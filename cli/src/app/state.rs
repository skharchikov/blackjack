use std::collections::VecDeque;

use bj_core::domain::engine::event::payload::EventPayload;
use crate::state::UiState;
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc;
use ulid::Ulid;

pub struct App {
    pub ui: UiState,
    pub should_quit: bool,
    pub server_url: String,
    pub player_id: String,
    pub ws_tx: Option<mpsc::Sender<String>>,
    pub current_table_id: Option<String>,
    pub table_min_bet: u32,
    pub table_max_bet: u32,
    /// Events waiting to be applied one-by-one for animation.
    pub event_queue: VecDeque<(u64, EventPayload)>,
    /// Tick counter used to pace event application.
    pub anim_tick: u64,
    /// Spinner animation state for the header.
    pub throbber_state: ThrobberState,
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
            table_min_bet: 10,
            table_max_bet: 1_000,
            event_queue: VecDeque::new(),
            anim_tick: 0,
            throbber_state: ThrobberState::default(),
        }
    }
}
