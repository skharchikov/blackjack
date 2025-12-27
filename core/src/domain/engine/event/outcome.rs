use crate::domain::player::PlayerId;

#[derive(Debug, Clone)]
pub enum PlayerOutcome {
    Won,
    Lost,
    Push,
    Blackjack,
    Bust,
}

#[derive(Debug, Clone)]
pub struct Payout {
    pub bet: u32,
    pub multiplier: f32, // 0.0 for loss, 1.0 for push, 2.0 for win, 2.5 for blackjack
}

impl Payout {
    pub fn new(bet: u32, multiplier: f32) -> Self {
        Self { bet, multiplier }
    }

    pub fn total(&self) -> u32 {
        (self.bet as f32 * self.multiplier) as u32
    }
}

#[derive(Debug, Clone)]
pub struct PlayerResult {
    pub player: PlayerId,
    pub outcome: PlayerOutcome,
    pub payout: Payout,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    pub player_results: Vec<PlayerResult>,
    pub dealer_busted: bool,
}
