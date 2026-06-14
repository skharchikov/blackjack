use std::collections::VecDeque;

use crate::state::UiState;
use bj_core::domain::engine::event::payload::EventPayload;
use tokio::{sync::mpsc, task::JoinHandle};
use ulid::Ulid;

pub struct App {
    pub ui: UiState,
    pub should_quit: bool,
    pub server_url: String,
    pub player_id: String,
    pub username: String,
    pub password: String,
    pub ws_tx: Option<mpsc::Sender<String>>,
    /// Handle to the active WS background task; awaited on clean shutdown.
    pub ws_task: Option<JoinHandle<()>>,
    pub current_table_id: Option<String>,
    pub table_min_bet: u32,
    pub table_max_bet: u32,
    pub event_queue: VecDeque<(u64, EventPayload)>,
    pub anim_tick: u64,
    /// Prevents spawning a new lobby poll while the previous one is in flight.
    pub lobby_poll_in_flight: bool,
    next_request_id: u64,
    /// Incremented each time a new WS task is spawned. Stale WsDisconnected/
    /// AuthFailed events carry the old generation and are ignored.
    pub ws_generation: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::login(),
            should_quit: false,
            server_url: std::env::var("SERVER_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".into()),
            player_id: Ulid::new().to_string(),
            username: String::new(),
            password: String::new(),
            ws_tx: None,
            ws_task: None,
            current_table_id: None,
            table_min_bet: 10,
            table_max_bet: 1_000,
            event_queue: VecDeque::new(),
            anim_tick: 0,
            lobby_poll_in_flight: false,
            next_request_id: 1,
            ws_generation: 0,
        }
    }
}

impl App {
    pub fn next_request_id(&mut self) -> u64 {
        let id = self.next_request_id;
        self.next_request_id += 1;
        id
    }
}
