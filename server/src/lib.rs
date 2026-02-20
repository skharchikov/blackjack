pub mod config;
pub mod routes;
pub mod store;

use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

/// Shared application state accessible from all handlers.
pub type AppState = Arc<App>;
pub type TableStore = Box<dyn store::TableStore>;

/// Application state.
pub struct App {
    pub connections: AtomicU64,
    pub table_store: TableStore,
}

impl App {
    pub fn new(table_store: impl store::TableStore + 'static) -> Self {
        Self {
            connections: AtomicU64::new(0),
            table_store: Box::new(table_store),
        }
    }
}

/// Messages sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Pong,
}
