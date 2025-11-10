use sqlx::{Error as SqlxError, migrate::MigrateError};
use std::{env::VarError, io::Error as IoError};
use thiserror::Error;

use serde_json::Error as SerdeJsonError;
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::ParsePubkeyError;

// Create an AppError using thiserror that handles almost all errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Axum Error: {0}")]
    AxumError(#[from] axum::Error),

    #[error("Solana Error: {0}")]
    SolanaError(String),

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Bad Request: {0}")]
    BadRequestError(String),
}

// Map the sqlx::Error to the DatabaseError variant of the AppError
impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        AppError::DatabaseError(e.to_string())
    }
}

// Map the sqlx::migrate::MigrateError to the DatabaseError variant of the AppError
impl From<MigrateError> for AppError {
    fn from(e: MigrateError) -> Self {
        AppError::DatabaseError(e.to_string())
    }
}

// Map the std::io::Error to the InternalError variant of the AppError
impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::InternalError(e.to_string())
    }
}

// Map the std::env::VarError to the InternalError variant of the AppError
impl From<VarError> for AppError {
    fn from(e: VarError) -> Self {
        AppError::InternalError(e.to_string())
    }
}

// Map the serde_json::Error to the InternalError variant of the AppError
impl From<SerdeJsonError> for AppError {
    fn from(e: SerdeJsonError) -> Self {
        AppError::InternalError(e.to_string())
    }
}

// Map the Solana ParsePubkeyError to the SolanaError variant of the AppError
impl From<ParsePubkeyError> for AppError {
    fn from(_: ParsePubkeyError) -> Self {
        AppError::SolanaError("Invalid Address".to_string())
    }
}

// Map the Solana ClientError to the SolanaError variant of the AppError
impl From<ClientError> for AppError {
    fn from(e: ClientError) -> Self {
        AppError::SolanaError(e.to_string())
    }
}
