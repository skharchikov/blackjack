pub mod dealer;
pub mod player;
pub mod system;

use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CommandId(pub u64);

pub trait CommandHandler {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError>;
}
