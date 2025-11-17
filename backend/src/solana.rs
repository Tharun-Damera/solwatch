use std::str::FromStr;

use axum::extract::ws::{Message, WebSocket};
use chrono::Utc;
use serde::Serialize;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use tracing::{Level, event, instrument};

use crate::{
    AppState,
    db::{
        accounts::{check_account_exists, insert_account},
        transactions::{insert_transactions, insert_transactions_signatures},
    },
    error::AppError,
    message::IndexingMessage,
    models::{Account, Transaction, TransactionSignature},
};

pub async fn send_message<T: Serialize>(socket: &mut WebSocket, value: T) {
    // Convert the passed-in value to a json string
    // then to the Message::Text for sending it to the client via socket
    match serde_json::to_string(&value) {
        Ok(msg) => {
            event!(Level::INFO, "Sending a message: {msg}");
            if let Err(e) = socket.send(Message::Text(msg.into())).await {
                event!(Level::ERROR, "Failed to send the message: {e}");
            }
        }
        Err(e) => {
            event!(Level::ERROR, "Failed to seriliaze message: {e}");
        }
    }
}

#[instrument(skip(socket, error))]
pub async fn send_error_message(socket: &mut WebSocket, error: AppError) {
    event!(Level::ERROR, "Error occurred: {error}");

    // Send the error message to the client
    send_message(
        socket,
        IndexingMessage::Error {
            message: &error.to_string(),
        },
    )
    .await;
}

#[instrument(skip(socket, state))]
pub async fn index_address(
    socket: &mut WebSocket,
    state: AppState,
    address: String,
) -> Result<(), AppError> {
    // Convert the address str to Address struct instance of Solana account
    let public_key = Pubkey::from_str(&address)?;

    // Before indexing the account, check if it is already indexed
    if check_account_exists(&state.db, &address).await {
        return Err(AppError::BadRequestError(
            "Account is already indexed".to_string(),
        ));
    }

    event!(Level::INFO, "Begin indexing the address");
    send_message(socket, IndexingMessage::Started { address: &address }).await;

    // Get the Solana account data of the address
    let account = state.rpc.get_account(&public_key).await?;
    event!(Level::INFO, ?account);

    let account = Account {
        address: address.clone(),
        lamports: account.lamports as i64,
        owner: account.owner.to_string(),
        executable: account.executable,
        data_length: account.data.len() as i64,
        rent_epoch: account.rent_epoch as i64,
        indexed_at: Utc::now().into(),
        last_updated_at: Utc::now().into(),
    };

    // Insert the account data into DB
    insert_account(&state.db, &account).await?;

    // Send the account data to the client via socket communication
    send_message(socket, IndexingMessage::AccountData { data: account }).await;

    // Get only the latest 20 transaction signatures
    let signatures = state
        .rpc
        .get_signatures_for_address_with_config(
            &public_key,
            GetConfirmedSignaturesForAddress2Config {
                before: None,
                until: None,
                limit: Some(20),
                commitment: None,
            },
        )
        .await?;
    event!(Level::INFO, ?signatures);

    if signatures.is_empty() {
        return Err(AppError::SolanaError(
            "No transactions found for this address".to_string(),
        ));
    }

    let mut txn_signs: Vec<TransactionSignature> = vec![];

    // // Parse the actual transaction signatures to DB format
    for sign in &signatures {
        txn_signs.push(TransactionSignature {
            signature: sign.signature.clone(),
            account_address: address.clone(),
            slot: sign.slot as i64,
            block_time: sign.block_time,
            confirmation_status: serde_json::from_str(&serde_json::to_string(
                &sign.confirmation_status,
            )?)?,
            indexed_at: Utc::now().into(),
        });
    }

    // Insert the transaction signatures into DB
    insert_transactions_signatures(&state.db, &txn_signs).await?;

    // Send the parsed transaction signatures to the client
    send_message(
        socket,
        IndexingMessage::TransactionSignatures { data: &txn_signs },
    )
    .await;

    let mut txns: Vec<Transaction> = vec![];
    for sign in &signatures {
        let signature = Signature::from_str(&sign.signature)?;

        let txn = state
            .rpc
            .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
            .await?;
        event!(Level::INFO, ?txn);

        txns.push(Transaction {
            signature: sign.signature.clone(),
            account_address: address.clone(),
            slot: txn.slot as i64,
            block_time: txn.block_time,
            transaction: serde_json::to_value(txn.transaction)?,
            indexed_at: Utc::now().into(),
        });
    }

    // Insert the transactions into DB
    insert_transactions(&state.db, &txns).await?;

    send_message(socket, IndexingMessage::Completed { address: &address }).await;
    event!(Level::INFO, "Indexing will now continue in the background");

    let next_signature = signatures.last().unwrap().signature.clone();

    // Spawn a task to index the rest of the account transaction signatures and transasctions
    let _bg_task = tokio::spawn(async move {
        if let Err(e) = continue_indexing(state, address, public_key, next_signature).await {
            event!(Level::ERROR, "Error Occurred in spawned task: {e:?}");
        }
    });

    Ok(())
}

#[instrument(skip(state, public_key, next_signature))]
async fn continue_indexing(
    state: AppState,
    address: String,
    public_key: Pubkey,
    next_signature: String,
) -> Result<(), AppError> {
    event!(Level::INFO, "Continue indexing the address ...");

    let mut total_txns = 0;
    let mut batch = 0;
    const BATCH_SIZE: usize = 1000;

    let mut before_signature = next_signature;

    loop {
        // Get the next batch transaction signatures
        let signatures = state
            .rpc
            .get_signatures_for_address_with_config(
                &public_key,
                GetConfirmedSignaturesForAddress2Config {
                    before: Some(Signature::from_str(&before_signature)?),
                    until: None,
                    limit: Some(BATCH_SIZE),
                    commitment: None,
                },
            )
            .await?;

        if signatures.is_empty() {
            event!(Level::INFO, "No more transactions found");
            break;
        }

        let mut txn_signs: Vec<TransactionSignature> = vec![];
        let mut txns: Vec<Transaction> = vec![];

        // Parse the transaction signatures to DB format
        for sign in &signatures {
            txn_signs.push(TransactionSignature {
                signature: sign.signature.clone(),
                account_address: address.clone(),
                slot: sign.slot as i64,
                block_time: sign.block_time,
                confirmation_status: serde_json::from_str(&serde_json::to_string(
                    &sign.confirmation_status,
                )?)?,
                indexed_at: Utc::now().into(),
            });

            let signature = Signature::from_str(&sign.signature)?;
            // Get the transaction details based on the signature
            let txn = state
                .rpc
                .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
                .await?;
            event!(Level::INFO, ?txn);

            txns.push(Transaction {
                signature: sign.signature.clone(),
                account_address: address.clone(),
                slot: txn.slot as i64,
                block_time: txn.block_time,
                transaction: serde_json::to_value(txn.transaction)?,
                indexed_at: Utc::now().into(),
            });
        }

        // Insert the transaction signatures into DB
        insert_transactions_signatures(&state.db, &txn_signs).await?;

        // Insert the transactions into DB
        insert_transactions(&state.db, &txns).await?;

        total_txns += signatures.len();
        batch += 1;
        event!(Level::INFO, total_txns, batch, "Batch completed");

        before_signature = signatures.last().unwrap().signature.clone();

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    event!(Level::INFO, "Indexing is completed");
    Ok(())
}
