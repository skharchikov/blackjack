use crate::domain::engine::command::{CommandHandler, CommandId};
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_id::GameId;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;

#[derive(Debug, Clone)]
pub struct DealerCommand {
    pub game_id: GameId,
    pub command_id: CommandId,
    pub action: DealerAction,
}

#[derive(Debug, Clone)]
pub enum DealerAction {}

impl CommandHandler for DealerAction {
    fn handle(
        &self,
        _state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match *self {}
    }
}
