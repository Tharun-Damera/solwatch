use axum::{
    extract::{
        Json, Path, Query, State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    db::{
        accounts::{check_account_exists, get_account},
        transactions::{get_transaction, get_transaction_signatures, get_transactions},
    },
    error::AppError,
    solana,
};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.clone(), address))
}

// Websocket handler that handles the Indexing of the Solana account based on the address
async fn handle_socket(mut socket: WebSocket, state: AppState, address: String) {
    if let Err(e) = solana::index_address(&mut socket, state, address.clone()).await {
        solana::send_error_message(&mut socket, &address, e).await;
    }
}

#[derive(Serialize)]
struct AccountStatus {
    indexed: bool,
}

// Entry point API of the app that checks whether the Solana account is indexed or not
#[instrument(skip(state))]
pub async fn get_account_status(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let state = state.clone();

    let indexed = check_account_exists(&state.db, &address).await;
    if indexed {
        Ok(Json(AccountStatus { indexed }))
    } else {
        Err(AppError::NotFoundError("Account Not found".to_string()))
    }
}

#[instrument(skip(state))]
pub async fn get_account_data(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let state = state.clone();

    if let Some(account) = get_account(&state.db, &address).await? {
        Ok(Json(account))
    } else {
        Err(AppError::NotFoundError("Account Not Found!".to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    skip: u64,
    limit: i64,
}

#[instrument(skip(state))]
pub async fn transaction_signatures(
    State(state): State<AppState>,
    Path(address): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let state = state.clone();

    let txns =
        get_transaction_signatures(&state.db, address, pagination.skip, pagination.limit).await?;

    Ok(Json(txns))
}

#[instrument(skip(state))]
pub async fn transactions(
    State(state): State<AppState>,
    Path(address): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let state = state.clone();

    let txns = get_transactions(&state.db, address, pagination.skip, pagination.limit).await?;

    Ok(Json(txns))
}

#[instrument(skip(state))]
pub async fn transaction_from_signature(
    State(state): State<AppState>,
    Path((address, signature)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let state = state.clone();

    if let Some(txn) = get_transaction(&state.db, address, signature).await? {
        Ok(Json(txn))
    } else {
        Err(AppError::NotFoundError(
            "Transaction Not Found!".to_string(),
        ))
    }
}
