use super::cards::UiHand;

#[derive(Debug, Clone)]
pub struct TableState {
    pub game_id: u64,
    pub phase: String,
    pub event_id: u64,
    pub dealer: UiHand,
    pub players: Vec<PlayerUiState>,
}

#[derive(Debug, Clone)]
pub struct PlayerUiState {
    pub name: String,
    pub active: bool,
    pub hand: UiHand,
    pub status: String, // "playing", "stood", "bust", etc.
}
