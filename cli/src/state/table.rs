use super::cards::UiHand;

#[derive(Debug, Clone)]
pub struct TableState {
    pub dealer: UiHand,
    pub players: Vec<PlayerUiState>,
}

#[derive(Debug, Clone)]
pub struct PlayerUiState {
    pub name: String,
    pub active: bool,
    pub hand: UiHand,
}
