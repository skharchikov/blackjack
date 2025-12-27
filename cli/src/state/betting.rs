#[derive(Debug, Clone)]
pub struct BettingState {
    pub min_bet: u64,
    pub max_bet: u64,
    pub current_bet: u64,
    pub step: u64,
    pub confirmed: bool,
}
