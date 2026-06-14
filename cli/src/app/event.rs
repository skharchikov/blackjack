use crate::state::lobby::TableSummary;
use crossterm::event::KeyCode;

/// All events that flow through the application's event channel.
#[derive(Debug)]
pub enum AppEvent {
    Key(KeyCode),
    Tick,
    Resize(u16, u16),
    LobbyRefreshed(Vec<TableSummary>),
    LobbyPollDone,
    WsMessage(String),
    WsConnected { player_id: String, generation: u64 },
    WsDisconnected { generation: u64 },
    AuthFailed { reason: String, generation: u64 },
    ServerError(String),
}
