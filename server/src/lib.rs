pub mod routes;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state accessible from all handlers.
pub type AppState = Arc<RwLock<App>>;

/// Application state.
#[derive(Debug, Default)]
pub struct App {
    pub connections: u64,
}

/// Messages sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Pong,
}
