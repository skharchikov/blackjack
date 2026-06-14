pub mod player_timeout;

pub use player_timeout::PlayerTimeout;

use crate::domain::engine::command::CommandHandler;
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;

#[derive(Debug, Clone)]
pub enum SystemCommand {
    PlayerTimeout(PlayerTimeout),
}

impl CommandHandler for SystemCommand {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match self {
            Self::PlayerTimeout(h) => h.handle(state, settings),
        }
    }
}
