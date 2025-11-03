use axum::{Router, routing::any};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new().route("/ws", any(handlers::websocket_handler))
}
