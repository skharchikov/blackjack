use crate::domain::engine::command::{CommandHandler, GameCommand};
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;

pub struct GameEngine;

impl GameEngine {
    pub fn handle(
        state: &GameState,
        settings: &TableSettings,
        cmd: &GameCommand,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match cmd {
            GameCommand::Player(c) => c.action.handle(state, settings),
            GameCommand::Dealer(c) => c.action.handle(state, settings),
            GameCommand::System(c) => c.handle(state, settings),
        }
    }
}
