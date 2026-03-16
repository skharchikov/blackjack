pub mod deal_initial_cards;
pub mod open_betting;
pub mod play_hand;
pub mod settle_round;

pub use deal_initial_cards::DealInitialCards;
pub use open_betting::OpenBetting;
pub use play_hand::PlayHand;
pub use settle_round::SettleRound;

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
pub enum DealerAction {
    OpenBetting(OpenBetting),
    DealInitialCards(DealInitialCards),
    PlayHand(PlayHand),
    SettleRound(SettleRound),
}

impl CommandHandler for DealerAction {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match self {
            Self::OpenBetting(h) => h.handle(state, settings),
            Self::DealInitialCards(h) => h.handle(state, settings),
            Self::PlayHand(h) => h.handle(state, settings),
            Self::SettleRound(h) => h.handle(state, settings),
        }
    }
}
