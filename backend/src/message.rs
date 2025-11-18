use serde::Serialize;

use crate::models::Account;

// Enum that is used in Websocket communication messages
// Server sends the messages to client based on the IndexingMessage enum variants
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum IndexingMessage {
    Started { address: String },
    AccountData { data: Account },
    TransactionSignatures { fetched: u64 },
    Completed { address: String },
    Error { message: String },
}
