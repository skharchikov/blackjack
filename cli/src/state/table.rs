use std::fmt;

use super::cards::UiHand;

#[derive(Debug, Clone)]
pub struct TableState {
    pub game_id: String,
    pub phase: GamePhase,
    pub event_seq: u64,
    pub dealer: UiHand,
    pub players: Vec<PlayerUiState>,
}

impl TableState {
    pub fn empty() -> Self {
        Self {
            game_id: String::new(),
            phase: GamePhase::WaitingForBets,
            event_seq: 0,
            dealer: UiHand {
                cards: vec![],
                value: None,
            },
            players: vec![],
        }
    }
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
            GamePhase::WaitingForBets | GamePhase::Betting => "Waiting for Bets",
            GamePhase::Dealing => "Dealing",
            GamePhase::PlayerTurn => "Player Turn",
            GamePhase::DealerTurn => "Dealer Turn",
            GamePhase::Resolving => "Settling",
            GamePhase::Finished => "Finished",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerUiState {
    pub player_id: String,
    pub name: String,
    pub active: bool,
    pub hand: UiHand,
    pub hand_value: u8,
    pub is_bust: bool,
    pub balance: u32,
    pub bet: Option<u32>,
    pub status: String,
}
