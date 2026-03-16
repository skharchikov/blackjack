use crate::domain::engine::command::CommandHandler;
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::player::PlayerId;
use crate::domain::table::TableSettings;

#[derive(Debug, Clone)]
pub struct PlayerTimeout {
    pub player_id: PlayerId,
}

impl CommandHandler for PlayerTimeout {
    fn handle(
        &self,
        _state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        todo!("player timeout handler")
    }
}
