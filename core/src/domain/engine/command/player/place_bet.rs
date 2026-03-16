use crate::domain::engine::command::CommandHandler;
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::player::PlayerId;
use crate::domain::table::TableSettings;

#[derive(Debug, Clone)]
pub struct PlaceBet {
    pub player_id: PlayerId,
    pub amount: u32,
}

impl CommandHandler for PlaceBet {
    fn handle(&self, _state: &GameState, _settings: &TableSettings) -> Result<Vec<EventPayload>, CommandError> {
        todo!("place bet handler")
    }
}
