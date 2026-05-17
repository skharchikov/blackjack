use crate::domain::player::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerOutcome {
    Won,
    Lost,
    Push,
    Blackjack,
    Bust,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PayoutMultiplier {
    Loss,
    Push,
    Win,
    Blackjack,
}

impl PayoutMultiplier {
    pub fn apply(&self, bet: u32) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Push => bet,
            Self::Win => bet * 2,
            Self::Blackjack => bet + bet / 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub bet: u32,
    pub multiplier: PayoutMultiplier,
}

impl Payout {
    pub fn new(bet: u32, multiplier: PayoutMultiplier) -> Self {
        Self { bet, multiplier }
    }
    pub fn total(&self) -> u32 {
        self.multiplier.apply(self.bet)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerResult {
    pub player: PlayerId,
    pub outcome: PlayerOutcome,
    pub payout: Payout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub player_results: Vec<PlayerResult>,
    pub dealer_busted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payout_loss() {
        assert_eq!(PayoutMultiplier::Loss.apply(100), 0);
    }
    #[test]
    fn payout_push() {
        assert_eq!(PayoutMultiplier::Push.apply(50), 50);
    }
    #[test]
    fn payout_win() {
        assert_eq!(PayoutMultiplier::Win.apply(100), 200);
    }
    #[test]
    fn payout_blackjack_even() {
        assert_eq!(PayoutMultiplier::Blackjack.apply(10), 15);
    }
    #[test]
    fn payout_blackjack_odd() {
        assert_eq!(PayoutMultiplier::Blackjack.apply(11), 16);
    }
}
