use crate::domain::{engine::action::PlayerDecision, hand::Hand, player::PlayerId};

#[derive(Debug)]
pub struct PlayerState {
    pub player_id: PlayerId,
    pub hand: Hand,
    pub balance: u32,
    pub bet: Option<u32>,
    pub decisions: Vec<PlayerDecision>,
}

impl PlayerState {
    pub fn new(player_id: PlayerId) -> Self {
        Self::with_balance(player_id, 0)
    }

    pub fn with_balance(player_id: PlayerId, balance: u32) -> Self {
        Self {
            player_id,
            hand: Hand::new(),
            balance,
            bet: None,
            decisions: Vec::new(),
        }
    }

    pub fn record_action(&mut self, action: PlayerDecision) {
        self.decisions.push(action);
    }

    pub fn place_bet(&mut self, amount: u32) -> Result<(), BetError> {
        if amount > self.balance {
            return Err(BetError::InsufficientFunds);
        }
        if amount == 0 {
            return Err(BetError::InvalidAmount);
        }

        self.balance -= amount;
        self.bet = Some(amount);
        Ok(())
    }

    pub fn clear_bet(&mut self) {
        self.bet = None;
    }

    pub fn add_winnings(&mut self, amount: u32) {
        self.balance += amount;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BetError {
    InsufficientFunds,
    InvalidAmount,
}
