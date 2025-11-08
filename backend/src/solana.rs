use std::str::FromStr;

use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;

use crate::{
    db::accounts::{insert_account, insert_transactions},
    error::AppError,
    message::IndexingMessage,
    models::{AccountCreate, TransactionSignatureCreate},
};

const DEV_NET: &str = "https://api.devnet.solana.com";

pub async fn send_message<T: Serialize>(socket: &mut WebSocket, value: T) {
    match serde_json::to_string(&value) {
        Ok(msg) => {
            if let Err(e) = socket.send(Message::Text(msg.into())).await {
                eprintln!("Failed to send message: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to seriliaze the message: {:?}", e);
        }
    }
}

pub async fn send_error_message(socket: &mut WebSocket, address: &str, error: AppError) {
    send_message(
        socket,
        IndexingMessage::Error {
            address,
            message: &error.to_string(),
        },
    )
    .await;
}

pub async fn index_address(
    socket: &mut WebSocket,
    pool: &PgPool,
    address: &str,
) -> Result<(), AppError> {
    let connection = RpcClient::new(DEV_NET);

    let public_key = Pubkey::from_str(address)?;
    send_message(socket, IndexingMessage::Started { address: &address }).await;

    let account = connection.get_account(&public_key)?;
    dbg!(&account);

    let inserted_acc = insert_account(
        pool,
        AccountCreate {
            address: address.to_string(),
            lamports: account.lamports as i64,
            owner: account.owner.to_string(),
            executable: account.executable,
            data_length: account.data.len() as i64,
            rent_epoch: account.rent_epoch as i64,
        },
    )
    .await?;

    send_message(socket, IndexingMessage::AccountData(inserted_acc)).await;

    let signatures = connection.get_signatures_for_address(&public_key)?;
    dbg!(&signatures);

    let mut txn_signs: Vec<TransactionSignatureCreate> = vec![];

    for sign in signatures {
        txn_signs.push(TransactionSignatureCreate {
            signature: sign.signature,
            account_address: address.to_string(),
            slot: sign.slot as i64,
            block_time: sign.block_time,
            confirmation_status: serde_json::from_str(&serde_json::to_string(
                &sign.confirmation_status,
            )?)?,
        });
    }
    dbg!(&txn_signs);

    insert_transactions(pool, &txn_signs).await?;
    send_message(
        socket,
        IndexingMessage::TransactionSignatures { data: &txn_signs },
    )
    .await;

    send_message(socket, IndexingMessage::Completed { address: &address }).await;

    Ok(())
}
