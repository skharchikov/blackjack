// Lobby-specific state will go here if needed
#[derive(Debug, Clone)]
pub struct LobbyState {
    pub status: LobbyStatus,
    pub tables: Vec<TableInfo>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub enum LobbyStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone)]
pub enum TableStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
    pub min_bet: u32,
    pub max_bet: u32,
    pub status: TableStatus,
}
