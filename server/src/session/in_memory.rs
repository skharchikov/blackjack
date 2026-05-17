use super::{CommandAck, GameSession, RequestId, SessionError, TableSummary};
use async_trait::async_trait;
use bj_core::domain::{
    engine::{command::player::PlayerAction, event::GameEvent, snapshot::GameStateSnapshot},
    PlayerId, TableId, TableSettings,
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use super::table_actor::TableHandle;

pub struct InMemoryGameSession {
    tables: Arc<DashMap<TableId, TableHandle>>,
}

impl InMemoryGameSession {
    pub fn new() -> Self {
        Self { tables: Arc::new(DashMap::new()) }
    }

    pub fn add_table(&self, id: TableId, name: String, settings: TableSettings) {
        let handle = TableHandle::spawn(id, name, settings);
        self.tables.insert(id, handle);
    }
}

impl Default for InMemoryGameSession {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl GameSession for InMemoryGameSession {
    async fn list_tables(&self) -> Vec<TableSummary> {
        let mut out = vec![];
        for entry in self.tables.iter() {
            if let Ok(s) = entry.value().summary().await {
                out.push(s);
            }
        }
        out
    }

    async fn snapshot(
        &self,
        table_id: TableId,
        player: PlayerId,
    ) -> Result<GameStateSnapshot, SessionError> {
        let handle = self.tables.get(&table_id).ok_or(SessionError::TableNotFound)?;
        handle.snapshot(player).await
    }

    async fn send_command(
        &self,
        table_id: TableId,
        player_id: PlayerId,
        request_id: RequestId,
        action: PlayerAction,
    ) -> Result<CommandAck, SessionError> {
        let handle = self.tables.get(&table_id).ok_or(SessionError::TableNotFound)?;
        handle.send_command(player_id, request_id, action).await
    }

    async fn subscribe(
        &self,
        table_id: TableId,
    ) -> Result<broadcast::Receiver<GameEvent>, SessionError> {
        let handle = self.tables.get(&table_id).ok_or(SessionError::TableNotFound)?;
        handle.subscribe()
    }
}
