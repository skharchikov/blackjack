use std::fmt;

use super::cards::UiHand;

#[derive(Debug, Clone)]
pub struct TableState {
    pub game_id: String,
    pub phase: GamePhase,
    pub event_seq: u64,
    pub dealer: UiHand,
    pub players: Vec<PlayerUiState>,
    pub observers: Vec<PlayerUiState>,
    pub waiting: Vec<PlayerUiState>,
    pub is_observer: bool,
    pub event_log: Vec<String>,
    /// True when it is this client's turn to act (hit/stand).
    pub is_my_turn: bool,
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
            observers: vec![],
            waiting: vec![],
            is_observer: false,
            event_log: vec![],
            is_my_turn: false,
        }
    }

    pub fn log(&mut self, msg: impl Into<String>) {
        self.event_log.push(msg.into());
        // Keep last 200 entries
        if self.event_log.len() > 200 {
            self.event_log.remove(0);
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
