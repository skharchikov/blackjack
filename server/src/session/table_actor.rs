use super::{CommandAck, RequestId, SessionError, TableSummary};
use bj_core::domain::{
    engine::{command::player::PlayerAction, event::GameEvent, snapshot::GameStateSnapshot},
    PlayerId, TableId, TableSettings,
};
use tokio::sync::broadcast;

pub struct TableHandle {
    id: TableId,
    name: String,
    settings: TableSettings,
    tx: broadcast::Sender<GameEvent>,
}

impl TableHandle {
    pub fn spawn(id: TableId, name: String, settings: TableSettings) -> Self {
        let (tx, _) = broadcast::channel(256);
        Self { id, name, settings, tx }
    }

    pub async fn summary(&self) -> Result<TableSummary, SessionError> {
        Ok(TableSummary {
            id: self.id,
            name: self.name.clone(),
            settings: self.settings.clone(),
            player_count: 0,
            phase: "WaitingForBets".to_string(),
            is_joinable: true,
        })
    }

    pub async fn snapshot(&self, _player: PlayerId) -> Result<GameStateSnapshot, SessionError> {
        Err(SessionError::Internal)
    }

    pub async fn send_command(
        &self,
        _player_id: PlayerId,
        request_id: RequestId,
        _action: PlayerAction,
    ) -> Result<CommandAck, SessionError> {
        Ok(CommandAck { request_id })
    }

    pub fn subscribe(&self) -> Result<broadcast::Receiver<GameEvent>, SessionError> {
        Ok(self.tx.subscribe())
    }
}
