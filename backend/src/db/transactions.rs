use futures::stream::TryStreamExt;
use mongodb::{Database, bson::doc};
use tracing::{Level, event};

use crate::error::AppError;
use crate::models::{Transaction, TransactionSignature};

const SIGNATURE_COLLECTION: &str = "transaction_signatures";
const TRANSACTION_COLLECTION: &str = "transactions";

pub async fn insert_transactions_signatures(
    db: &Database,
    signatures: &[TransactionSignature],
) -> Result<(), AppError> {
    let inserted = db
        .collection::<TransactionSignature>(SIGNATURE_COLLECTION)
        .insert_many(signatures)
        .await?;
    event!(Level::INFO, ?inserted);

    Ok(())
}

pub async fn insert_transactions(db: &Database, txns: &[Transaction]) -> Result<(), AppError> {
    let inserted = db
        .collection::<Transaction>(TRANSACTION_COLLECTION)
        .insert_many(txns)
        .await?;
    event!(Level::INFO, ?inserted);

    Ok(())
}

pub async fn get_transaction_signatures(
    db: &Database,
    address: String,
    skip: u64,
    limit: i64,
) -> Result<Vec<TransactionSignature>, AppError> {
    let signatures: Vec<TransactionSignature> = db
        .collection::<TransactionSignature>(SIGNATURE_COLLECTION)
        .find(doc! {"account_address": address})
        .sort(doc! {"slot": -1})
        .skip(skip)
        .limit(limit)
        .await?
        .try_collect()
        .await?;
    event!(Level::INFO, ?signatures);

    Ok(signatures)
}

pub async fn get_transactions(
    db: &Database,
    address: String,
    skip: u64,
    limit: i64,
) -> Result<Vec<Transaction>, AppError> {
    let signatures: Vec<Transaction> = db
        .collection::<Transaction>(TRANSACTION_COLLECTION)
        .find(doc! {"account_address": address})
        .sort(doc! {"slot": -1})
        .skip(skip)
        .limit(limit)
        .await?
        .try_collect()
        .await?;
    event!(Level::INFO, ?signatures);

    Ok(signatures)
}
