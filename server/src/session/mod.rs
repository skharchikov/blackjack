pub mod in_memory;
pub mod summary;
pub mod table_actor;

use async_trait::async_trait;
use bj_core::domain::{
    engine::{command::player::PlayerAction, event::GameEvent, snapshot::GameStateSnapshot},
    PlayerId, TableId,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::broadcast;

pub use summary::TableSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub u64);

#[derive(Debug, Clone)]
pub struct CommandAck {
    pub request_id: RequestId,
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("table not found")]
    TableNotFound,
    #[error("command rejected: {0}")]
    CommandRejected(String),
    #[error("internal error")]
    Internal,
}

#[async_trait]
pub trait GameSession: Send + Sync {
    async fn list_tables(&self) -> Vec<TableSummary>;
    async fn snapshot(
        &self,
        table_id: TableId,
        player: PlayerId,
    ) -> Result<GameStateSnapshot, SessionError>;
    async fn send_command(
        &self,
        table_id: TableId,
        player_id: PlayerId,
        request_id: RequestId,
        action: PlayerAction,
    ) -> Result<CommandAck, SessionError>;
    async fn subscribe(
        &self,
        table_id: TableId,
    ) -> Result<broadcast::Receiver<GameEvent>, SessionError>;
}
