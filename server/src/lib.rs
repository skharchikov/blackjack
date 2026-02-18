pub mod config;
pub mod routes;
pub mod store;

use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use store::PostgresTableStore;

/// Shared application state accessible from all handlers.
pub type AppState = Arc<App>;

/// Application state.
pub struct App {
    pub connections: AtomicU64,
    pub table_store: PostgresTableStore,
}

impl App {
    pub fn new(table_store: PostgresTableStore) -> Self {
        Self {
            connections: AtomicU64::new(0),
            table_store,
        }
    }
}

/// Messages sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Pong,
}
