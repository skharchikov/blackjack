pub mod event;
pub mod keys;
pub mod state;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self as ct_event, Event};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::ui::render;
use event::AppEvent;
use keys::handle_key;
use state::App;

pub async fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let (tx, mut rx) = mpsc::channel::<AppEvent>(64);

    // Input reader
    let tx_input = tx.clone();
    tokio::task::spawn_blocking(move || loop {
        match ct_event::poll(Duration::from_millis(250)) {
            Err(_) => break,
            Ok(false) => continue,
            Ok(true) => {}
        }
        if let Ok(event) = ct_event::read() {
            let app_event = match event {
                Event::Key(k) if k.kind == ct_event::KeyEventKind::Press => {
                    Some(AppEvent::Key(k.code))
                }
                Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
                _ => None,
            };
            if let Some(evt) = app_event {
                if tx_input.blocking_send(evt).is_err() {
                    break;
                }
            }
        }
    });

    // Tick timer
    let tx_tick = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            match tx_tick.try_send(AppEvent::Tick) {
                Ok(_) | Err(mpsc::error::TrySendError::Full(_)) => {}
                Err(mpsc::error::TrySendError::Closed(_)) => break,
            }
        }
    });

    let mut tick_count: u64 = 0;

    loop {
        terminal.draw(|f| render(f, &app.ui))?;

        if let Some(event) = rx.recv().await {
            match event {
                AppEvent::Key(key) => handle_key(&mut app, key, &tx),
                AppEvent::Tick => {
                    tick_count += 1;
                    // Poll lobby every ~3s (12 ticks × 250ms)
                    if tick_count % 12 == 0 {
                        if let crate::state::Screen::Lobby(_) = &app.ui.screen {
                            let url = format!("{}/tables", app.server_url);
                            let tx2 = tx.clone();
                            tokio::spawn(async move {
                                if let Ok(resp) = reqwest::get(&url).await {
                                    if let Ok(list) =
                                        resp.json::<Vec<crate::state::lobby::TableSummary>>().await
                                    {
                                        let _ = tx2.send(AppEvent::LobbyRefreshed(list)).await;
                                    }
                                }
                            });
                        }
                    }
                }
                AppEvent::LobbyRefreshed(tables) => {
                    if let crate::state::Screen::Lobby(ref mut lobby) = app.ui.screen {
                        let selected = lobby.selected.min(tables.len().saturating_sub(1));
                        lobby.tables = tables;
                        lobby.selected = selected;
                        lobby.status = crate::state::lobby::LobbyStatus::Connected;
                    }
                }
                AppEvent::WsConnected { player_id } => {
                    tracing::info!("WS connected as {player_id}");
                }
                AppEvent::WsMessage(json) => {
                    handle_ws_message(&mut app, json);
                }
                AppEvent::WsDisconnected => {
                    app.ws_tx = None;
                    app.current_table_id = None;
                    app.ui = crate::state::UiState::lobby();
                }
                AppEvent::ServerError(e) => {
                    tracing::error!("server error: {e}");
                }
                AppEvent::Resize(..) => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

pub fn spawn_ws(app: &mut App, table_id: String, tx: &mpsc::Sender<AppEvent>) {
    let ws_url = format!("{}/ws", app.server_url.replace("http", "ws"));
    let player_id = app.player_id.clone();
    let tx_app = tx.clone();
    let (ws_cmd_tx, mut ws_cmd_rx) = mpsc::channel::<String>(32);
    app.ws_tx = Some(ws_cmd_tx);

    tokio::spawn(async move {
        use futures_util::{SinkExt, StreamExt};
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::Message;

        let (mut ws, _) = match connect_async(&ws_url).await {
            Ok(v) => v,
            Err(e) => {
                let _ = tx_app.send(AppEvent::ServerError(e.to_string())).await;
                return;
            }
        };

        // Auth
        let auth = serde_json::json!({"type": "Auth", "player_id": player_id});
        if ws
            .send(Message::Text(auth.to_string().into()))
            .await
            .is_err()
        {
            return;
        }

        // Wait for AuthOk
        loop {
            match ws.next().await {
                Some(Ok(Message::Text(t))) => {
                    if t.contains("AuthOk") {
                        let _ = tx_app
                            .send(AppEvent::WsConnected {
                                player_id: player_id.clone(),
                            })
                            .await;
                        break;
                    }
                }
                _ => return,
            }
        }

        // JoinTable
        let join = serde_json::json!({"type": "JoinTable", "table_id": table_id, "request_id": 1});
        if ws
            .send(Message::Text(join.to_string().into()))
            .await
            .is_err()
        {
            return;
        }

        // Forward loop
        loop {
            tokio::select! {
                Some(cmd) = ws_cmd_rx.recv() => {
                    if ws.send(Message::Text(cmd.into())).await.is_err() {
                        break;
                    }
                }
                msg = ws.next() => {
                    match msg {
                        Some(Ok(Message::Text(t))) => {
                            let _ = tx_app.send(AppEvent::WsMessage(t.to_string())).await;
                        }
                        None | Some(Err(_)) | Some(Ok(Message::Close(_))) => break,
                        _ => {}
                    }
                }
            }
        }

        let _ = tx_app.send(AppEvent::WsDisconnected).await;
    });
}

fn handle_ws_message(app: &mut App, json: String) {
    use serde_json::Value;
    let Ok(v) = serde_json::from_str::<Value>(&json) else {
        return;
    };
    let msg_type = v["type"].as_str().unwrap_or("");

    match msg_type {
        "Snapshot" => {
            if let Some(tid) = v["table_id"].as_str() {
                app.current_table_id = Some(tid.to_string());
            }
            app.ui = crate::state::UiState::table_view();
        }
        "Event" => {
            if let crate::state::Screen::Table(ref mut _table) = app.ui.screen {
                tracing::debug!("game event: {}", &json[..json.len().min(120)]);
            }
        }
        "CommandError" => {
            tracing::warn!("command error: {json}");
        }
        _ => {}
    }
}
