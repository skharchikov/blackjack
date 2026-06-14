pub mod join_table;
pub mod leave_seat;
pub mod leave_table;
pub mod take_seat;

pub use join_table::JoinTable;
pub use leave_seat::LeaveSeat;
pub use leave_table::LeaveTable;
pub use take_seat::TakeSeat;

use crate::domain::engine::command::{CommandHandler, CommandId};
use crate::domain::engine::error::CommandError;
use crate::domain::engine::event::payload::EventPayload;
use crate::domain::engine::game_id::GameId;
use crate::domain::engine::game_state::GameState;
use crate::domain::table::TableSettings;

#[derive(Debug, Clone)]
pub struct PlayerCommand {
    pub game_id: GameId,
    pub command_id: CommandId,
    pub action: PlayerAction,
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    JoinTable(JoinTable),
    LeaveSeat(LeaveSeat),
    LeaveTable(LeaveTable),
    TakeSeat(TakeSeat),
}

impl CommandHandler for PlayerAction {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match self {
            Self::JoinTable(h) => h.handle(state, settings),
            Self::LeaveSeat(h) => h.handle(state, settings),
            Self::LeaveTable(h) => h.handle(state, settings),
            Self::TakeSeat(h) => h.handle(state, settings),
        }
    }
}

impl CommandHandler for PlayerCommand {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        self.action.handle(state, settings)
    }
}
