use crossterm::event::KeyCode;
use tokio::sync::mpsc;

use crate::state::{GamePhase, LoginField, LoginStatus, Screen, UiState};

use super::event::AppEvent;
use super::state::App;

pub fn handle_key(app: &mut App, key: KeyCode, tx: &mpsc::Sender<AppEvent>) {
    if let KeyCode::Char('q') = key {
        // Don't quit on 'q' when typing in login fields
        if let Screen::Login(_) = &app.ui.screen {
            handle_login_key(app, key, tx);
            return;
        }
        app.should_quit = true;
        return;
    }

    match &app.ui.screen {
        Screen::Login(_) => handle_login_key(app, key, tx),
        Screen::Lobby(_) => handle_lobby_key(app, key, tx),
        Screen::Table(_) => handle_table_key(app, key, tx),
    }
}

fn handle_login_key(app: &mut App, key: KeyCode, _tx: &mpsc::Sender<AppEvent>) {
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

fn handle_lobby_key(app: &mut App, key: KeyCode, tx: &mpsc::Sender<AppEvent>) {
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
            if let Some(table) = lobby.tables.get(lobby.selected) {
                if table.is_joinable {
                    let table_id = table.id.clone();
                    app.table_min_bet = table.settings.min_bet;
                    app.table_max_bet = table.settings.max_bet;
                    crate::app::spawn_ws(app, table_id, tx);
                }
            }
        }
        _ => {}
    }
}

fn handle_table_key(app: &mut App, key: KeyCode, tx: &mpsc::Sender<AppEvent>) {
    let Screen::Table(ref table) = app.ui.screen else {
        return;
    };
    let phase = table.phase;

    // Leave works from any phase
    if let KeyCode::Char('l') = key {
        if let (Some(ref ws_tx), Some(ref tid)) = (&app.ws_tx, &app.current_table_id) {
            let msg = serde_json::json!({"type": "LeaveTable", "table_id": tid, "request_id": 99});
            let _ = ws_tx.try_send(msg.to_string());
        }
        app.ws_tx = None;
        app.current_table_id = None;
        app.ui = crate::state::UiState::lobby();
        return;
    }

    if phase == GamePhase::Betting {
        handle_betting_key(app, key);
        return;
    }

    // PlayerTurn actions
    if phase == GamePhase::PlayerTurn {
        match key {
            KeyCode::Char('h') => {
                if let (Some(ref ws_tx), Some(ref tid)) = (&app.ws_tx, &app.current_table_id) {
                    let msg = serde_json::json!({"type": "Hit", "table_id": tid, "request_id": 2});
                    let _ = ws_tx.try_send(msg.to_string());
                }
            }
            KeyCode::Char('s') => {
                if let (Some(ref ws_tx), Some(ref tid)) = (&app.ws_tx, &app.current_table_id) {
                    let msg =
                        serde_json::json!({"type": "Stand", "table_id": tid, "request_id": 3});
                    let _ = ws_tx.try_send(msg.to_string());
                }
            }
            _ => {}
        }
    }

    // suppress unused warning: tx is threaded through for future use
    let _ = tx;
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
            let amount = betting.current_bet;
            betting.confirmed = true;
            if let (Some(ref ws_tx), Some(ref tid)) = (&app.ws_tx, &app.current_table_id) {
                let msg = serde_json::json!({"type": "PlaceBet", "table_id": tid, "request_id": 10, "amount": amount});
                let _ = ws_tx.try_send(msg.to_string());
            }
        }
        _ => {}
    }
}
