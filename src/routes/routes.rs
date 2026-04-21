use std::sync::Arc;

use axum::{ Router, routing::{ get, post } };
use mongodb::Database;

use crate::routes::{ close_round, create_round, get_ticket };

pub fn routes(db: Database) -> Router {
    let db = Arc::new(db);
    Router::new()
        .route("/round/start", post(create_round))
        .route("/round/close", post(close_round))
        .route("/round/ticket", get(get_ticket))
        .route("/health", get(hello))
        .with_state(db)
}

pub async fn hello() -> &'static str {
    "Hello from Rust 🚀"
}
