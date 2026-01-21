use std::fmt;

use super::cards::UiHand;

#[derive(Debug, Clone)]
pub struct TableState {
    pub game_id: u64,
    pub phase: GamePhase,
    pub event_id: u64,
    pub dealer: UiHand,
    pub players: Vec<PlayerUiState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    WaitingForBets,
    Betting,
    Dealing,
    PlayerTurn,
    DealerTurn,
    Resolving,
    Finished,
}

impl fmt::Display for GamePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GamePhase::WaitingForBets => "Waiting for Bets",
            GamePhase::Betting => "Betting",
            GamePhase::Dealing => "Dealing",
            GamePhase::PlayerTurn => "Player Turn",
            GamePhase::DealerTurn => "Dealer Turn",
            GamePhase::Resolving => "Resolving",
            GamePhase::Finished => "Finished",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerUiState {
    pub name: String,
    pub active: bool,
    pub hand: UiHand,
    pub status: String, // "playing", "stood", "bust", etc.
}
