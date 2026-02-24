mod health;
mod table;
mod ws;

use axum::{routing::get, Router};

use crate::AppState;

/// Create the application router with all routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/tables", get(table::list_tables))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
}
