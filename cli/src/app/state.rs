use std::collections::VecDeque;

use bj_core::domain::engine::event::payload::EventPayload;
use crate::state::UiState;
use tokio::sync::mpsc;
use ulid::Ulid;

pub struct App {
    pub ui: UiState,
    pub should_quit: bool,
    pub server_url: String,
    pub player_id: String,
    pub username: String,
    pub password: String,
    pub ws_tx: Option<mpsc::Sender<String>>,
    pub current_table_id: Option<String>,
    pub table_min_bet: u32,
    pub table_max_bet: u32,
    pub event_queue: VecDeque<(u64, EventPayload)>,
    pub anim_tick: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::login(),
            should_quit: false,
            server_url: "http://127.0.0.1:3000".into(),
            player_id: Ulid::new().to_string(),
            username: String::new(),
            password: String::new(),
            ws_tx: None,
            current_table_id: None,
            table_min_bet: 10,
            table_max_bet: 1_000,
            event_queue: VecDeque::new(),
            anim_tick: 0,
        }
    }
}
