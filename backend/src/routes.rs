use axum::{
    Router,
    routing::{any, get},
};
use sqlx::PgPool;

use crate::handlers::*;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/ws/index/{address}", any(websocket_handler))
        .route("/api/account/{address}/status", get(get_account_status))
        .with_state(pool)
}
