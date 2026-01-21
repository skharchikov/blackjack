use crossterm::event::KeyCode;

use crate::state::{GamePhase, LobbyStatus, LoginStatus, Screen, UiState};

pub struct App {
    pub ui: UiState,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::login(),
        }
    }

    pub fn on_key(&mut self, key: KeyCode) -> bool {
        // Global quit
        if let KeyCode::Char('q') = key {
            return true;
        }

        // Handle screen-specific keys
        match &self.ui.screen {
            Screen::Login(_) => self.on_login_key(key),
            Screen::Lobby(_) => self.on_lobby_key(key),
            Screen::Table(_) => self.on_table_key(key),
        }
    }

    fn on_login_key(&mut self, key: KeyCode) -> bool {
        let Screen::Login(ref mut login) = self.ui.screen else {
            return false;
        };

        match key {
            KeyCode::Char(c) => {
                login.username.push(c);
            }
            KeyCode::Backspace => {
                login.username.pop();
            }
            KeyCode::Enter => {
                if !login.username.is_empty() {
                    login.status = LoginStatus::Connecting;
                    // Simulate login success -> go to lobby
                    self.ui = UiState::lobby();
                }
            }
            _ => {}
        }

        false
    }

    fn on_lobby_key(&mut self, key: KeyCode) -> bool {
        let Screen::Lobby(ref mut lobby) = self.ui.screen else {
            return false;
        };

        match key {
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

    fn on_table_key(&mut self, key: KeyCode) -> bool {
        let Screen::Table(ref table) = self.ui.screen else {
            return false;
        };

        match table.phase {
            GamePhase::Betting => self.on_betting_key(key),
            _ => false,
        }
    }

    fn on_betting_key(&mut self, key: KeyCode) -> bool {
        let betting = match self.ui.betting.as_mut() {
            Some(b) => b,
            None => return false,
        };

        match key {
            KeyCode::Left => {
                betting.current_bet = betting
                    .current_bet
                    .saturating_sub(betting.step)
                    .max(betting.min_bet);
            }
            KeyCode::Right => {
                betting.current_bet = (betting.current_bet + betting.step).min(betting.max_bet);
            }
            KeyCode::Enter => {
                betting.confirmed = true;
            }
            _ => {}
        }

        false
    }

    fn enter_table(&mut self) {
        self.ui = UiState::betting();
    }
}
