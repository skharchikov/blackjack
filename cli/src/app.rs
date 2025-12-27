use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

use crate::animation::DealAnimation;
use crate::mock::{deal_step_ui, mock_lobby_ui, mock_player_turn_ui, mock_resolving_ui};
use crate::state::lobby::LobbyStatus;
use crate::state::UiState;

pub struct App {
    pub ui: UiState,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::lobby(),
        }
    }

    pub fn on_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => return true,
            KeyCode::Char('d') => self.start_deal_animation(),
            KeyCode::Char('l') => self.ui = mock_lobby_ui(),
            KeyCode::Char('p') => self.ui = mock_player_turn_ui(),
            KeyCode::Char('r') => self.ui = mock_resolving_ui(),
            _ => {}
        }
        false
    }

    fn on_lobby_key(&mut self, key: KeyCode) -> bool {
        let lobby = self.ui.lobby.as_mut().unwrap();

        match key {
            KeyCode::Char('q') => return true,

            KeyCode::Up => {
                if lobby.selected > 0 {
                    lobby.selected -= 1;
                }
            }

            KeyCode::Down => {
                if lobby.selected + 1 < lobby.tables.len() {
                    lobby.selected += 1;
                }
            }

            KeyCode::Enter => {
                lobby.status = LobbyStatus::Connecting;

                self.enter_table();
            }

            _ => {}
        }

        false
    }

    fn start_deal_animation(&mut self) {
        self.ui = deal_step_ui(0);
        self.ui.deal_animation = Some(DealAnimation {
            step: 0,
            last_tick: Instant::now(),
        });
    }

    pub fn update_animation(&mut self) {
        let Some(anim) = self.ui.deal_animation.as_mut() else {
            return;
        };

        if anim.last_tick.elapsed() < Duration::from_millis(500) {
            return;
        }

        anim.step += 1;
        anim.last_tick = Instant::now();

        let step = anim.step;

        if step > 4 {
            self.ui = mock_player_turn_ui();
        } else {
            self.ui = deal_step_ui(step);
            self.ui.deal_animation = Some(DealAnimation {
                step,
                last_tick: Instant::now(),
            });
        }
    }

    fn enter_table(&mut self) {
        self.ui = mock_player_turn_ui();
    }
}
