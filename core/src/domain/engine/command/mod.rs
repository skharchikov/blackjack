use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;

pub mod dealer;
pub mod player;
pub mod system;

pub use dealer::{DealerAction, DealerCommand};
pub use player::{PlayerAction, PlayerCommand};
pub use system::SystemCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandId(pub u64);

pub trait CommandHandler {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError>;
}

#[derive(Debug, Clone)]
pub enum GameCommand {
    Player(PlayerCommand),
    Dealer(DealerCommand),
    System(SystemCommand),
}
