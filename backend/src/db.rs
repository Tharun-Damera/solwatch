use mongodb::{Client, Database};

use crate::error::AppError;

pub mod accounts;
pub mod transactions;

pub async fn init() -> Result<Database, AppError> {
    // Get the Mongo URI and DB name from the env
    let uri = std::env::var("MONGO_URI").expect("MONGO_URI env variable is mising");
    let db = std::env::var("MONGO_DB").expect("MONGO_DB env variable is mising");

    // Setup the Mongo Database
    let db = Client::with_uri_str(uri).await?.database(&db);

    Ok(db)
}
