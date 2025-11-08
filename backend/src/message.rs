use serde::Serialize;

use crate::models::{Account, TransactionSignatureCreate};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum IndexingMessage<'a> {
    Started {
        address: &'a str,
    },
    AccountData(Account),
    TransactionSignatures {
        data: &'a [TransactionSignatureCreate],
    },
    Completed {
        address: &'a str,
    },
    Error {
        address: &'a str,
        message: &'a str,
    },
}
