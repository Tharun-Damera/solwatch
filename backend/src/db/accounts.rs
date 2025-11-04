use sqlx::PgPool;

use crate::models::Accounts;

pub async fn check_account_exists(pool: &PgPool, address: String) -> bool {
    let account = sqlx::query_as!(Accounts, "SELECT * FROM accounts WHERE address=$1", address)
        .fetch_one(pool)
        .await;
    match account {
        Ok(acc) => {
            println!("{:?}", acc);
            true
        }
        Err(e) => {
            println!("Error: {:?}", e);
            false
        }
    }
}
