use crate::state::UiState;
use tokio::sync::mpsc;
use ulid::Ulid;

pub struct App {
    pub ui: UiState,
    pub should_quit: bool,
    pub server_url: String,
    pub player_id: String,
    pub ws_tx: Option<mpsc::Sender<String>>,
    pub current_table_id: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::login(),
            should_quit: false,
            server_url: "http://127.0.0.1:3000".into(),
            player_id: Ulid::new().to_string(),
            ws_tx: None,
            current_table_id: None,
        }
    }
}
