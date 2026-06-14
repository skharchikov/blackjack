use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TableSummary {
    pub id: String,
    pub name: String,
    pub player_count: usize,
    pub phase: String,
    pub is_joinable: bool,
    pub settings: TableSummarySettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TableSummarySettings {
    pub min_bet: u32,
    pub max_bet: u32,
    pub max_players: usize,
}

#[derive(Debug, Clone)]
pub struct LobbyState {
    pub status: LobbyStatus,
    pub tables: Vec<TableSummary>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub enum LobbyStatus {
    Disconnected,
    Loading,
    Connected,
}
