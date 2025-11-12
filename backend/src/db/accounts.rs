use mongodb::{Database, bson::doc};
use tracing::{Level, event};

use crate::error::AppError;
use crate::models::Account;

pub async fn get_account(db: &Database, address: &str) -> Result<Option<Account>, AppError> {
    let account = db
        .collection::<Account>("accounts")
        .find_one(doc! {"_id": address})
        .await?;

    Ok(account)
}

pub async fn check_account_exists(db: &Database, address: &str) -> bool {
    match get_account(db, address).await {
        Ok(acc) => match acc {
            Some(account) => {
                event!(Level::INFO, "Account Found: {account:?}");
                true
            }
            None => {
                event!(Level::INFO, "Account Not Found");
                false
            }
        },
        Err(e) => {
            event!(Level::ERROR, "Error occurred while finding account: {e:?}");
            false
        }
    }
}

pub async fn insert_account(db: &Database, account: &Account) -> Result<(), AppError> {
    let inserted = db
        .collection::<Account>("accounts")
        .insert_one(account)
        .await?;
    event!(Level::INFO, ?inserted);

    Ok(())
}
