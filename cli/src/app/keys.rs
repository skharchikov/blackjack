use crossterm::event::KeyCode;

use crate::state::{GamePhase, LobbyStatus, LoginField, LoginStatus, Screen, UiState};

use super::state::App;

pub fn handle_key(app: &mut App, key: KeyCode) {
    if let KeyCode::Char('q') = key {
        // Don't quit on 'q' when typing in login fields
        if let Screen::Login(_) = &app.ui.screen {
            handle_login_key(app, key);
            return;
        }
        app.should_quit = true;
        return;
    }

    match &app.ui.screen {
        Screen::Login(_) => handle_login_key(app, key),
        Screen::Lobby(_) => handle_lobby_key(app, key),
        Screen::Table(_) => handle_table_key(app, key),
    }
}

fn handle_login_key(app: &mut App, key: KeyCode) {
    let Screen::Login(ref mut login) = app.ui.screen else {
        return;
    };

    match key {
        KeyCode::Tab | KeyCode::BackTab => {
            login.active_field = match login.active_field {
                LoginField::Username => LoginField::Password,
                LoginField::Password => LoginField::Username,
            };
        }
        KeyCode::Char(c) => match login.active_field {
            LoginField::Username => login.username.push(c),
            LoginField::Password => login.password.push(c),
        },
        KeyCode::Backspace => match login.active_field {
            LoginField::Username => {
                login.username.pop();
            }
            LoginField::Password => {
                login.password.pop();
            }
        },
        KeyCode::Enter => {
            if !login.username.is_empty() && !login.password.is_empty() {
                login.status = LoginStatus::Connecting;
                app.ui = UiState::lobby();
            }
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
}

fn handle_lobby_key(app: &mut App, key: KeyCode) {
    let Screen::Lobby(ref mut lobby) = app.ui.screen else {
        return;
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
            app.ui = UiState::betting();
        }
        _ => {}
    }
}

fn handle_table_key(app: &mut App, key: KeyCode) {
    let Screen::Table(ref table) = app.ui.screen else {
        return;
    };

    if table.phase == GamePhase::Betting {
        handle_betting_key(app, key);
    }
}

fn handle_betting_key(app: &mut App, key: KeyCode) {
    let Some(betting) = app.ui.betting.as_mut() else {
        return;
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
}
