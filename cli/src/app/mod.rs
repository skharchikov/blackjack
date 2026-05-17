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
                    app.player_id = player_id;
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

        // Wait for AuthOk — capture server-assigned player_id
        let confirmed_player_id = loop {
            match ws.next().await {
                Some(Ok(Message::Text(t))) => {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&t) {
                        if v["type"].as_str() == Some("AuthOk") {
                            let pid = v["player_id"]
                                .as_str()
                                .unwrap_or(&player_id)
                                .to_string();
                            break pid;
                        }
                    }
                }
                _ => return,
            }
        };

        let _ = tx_app
            .send(AppEvent::WsConnected {
                player_id: confirmed_player_id,
            })
            .await;

        // JoinTable
        let join =
            serde_json::json!({"type": "JoinTable", "table_id": table_id, "request_id": 1});
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
            use bj_core::domain::engine::snapshot::GameStateSnapshot;
            if let Ok(snap) =
                serde_json::from_value::<GameStateSnapshot>(v["state"].clone())
            {
                let table = table_state_from_snapshot(&snap, &app.player_id);
                app.ui = crate::state::UiState::from_table_state(
                    table,
                    app.table_min_bet,
                    app.table_max_bet,
                );
            }
        }
        "Event" => {
            let seq = v["event"]["seq"].as_u64().unwrap_or(0);
            if let Some(payload_val) = v.get("event").and_then(|e| e.get("payload")) {
                use bj_core::domain::engine::event::payload::EventPayload;
                if let Ok(payload) =
                    serde_json::from_value::<EventPayload>(payload_val.clone())
                {
                    apply_event_payload(app, payload, seq);
                }
            }
        }
        "CommandError" => {
            tracing::warn!("command error: {json}");
        }
        _ => {}
    }
}

fn table_state_from_snapshot(
    snap: &bj_core::domain::engine::snapshot::GameStateSnapshot,
    my_player_id: &str,
) -> crate::state::table::TableState {
    use bj_core::domain::engine::phase::Phase;
    use crate::state::{
        cards::{UiCard, UiHand},
        table::{PlayerUiState, TableState},
    };

    let phase = server_phase_to_game_phase(&snap.phase);
    let active_pid = if let Phase::PlayerTurn(pid) = &snap.phase {
        Some(pid.to_string())
    } else {
        None
    };

    let players = snap
        .players
        .iter()
        .map(|p| {
            let pid = p.player_id.to_string();
            let is_active = active_pid.as_ref().map(|a| a == &pid).unwrap_or(false);
            let cards: Vec<UiCard> = p
                .cards
                .iter()
                .map(|c| UiCard::visible(*c))
                .collect();
            let hand = UiHand {
                value: Some(p.hand_value.to_string()),
                cards,
            };
            let status = if p.is_bust {
                "BUST".into()
            } else if p.bet.is_some() {
                "bet placed".into()
            } else {
                "waiting".into()
            };
            PlayerUiState {
                name: short_id(&pid),
                active: is_active,
                hand,
                hand_value: p.hand_value,
                is_bust: p.is_bust,
                balance: p.balance,
                bet: p.bet,
                status,
                player_id: pid,
            }
        })
        .collect();

    let dealer_cards: Vec<UiCard> = snap
        .dealer
        .cards
        .iter()
        .map(|opt| match opt {
            Some(c) => UiCard::visible(*c),
            None => UiCard::hidden(),
        })
        .collect();
    let dealer_value = {
        let hand = UiHand {
            cards: dealer_cards.clone(),
            value: None,
        };
        let v = hand.compute_value();
        if v > 0 { Some(v.to_string()) } else { None }
    };

    let _ = my_player_id;

    let mut state = TableState {
        game_id: snap.game_id.to_string(),
        phase,
        event_seq: 0,
        dealer: UiHand {
            cards: dealer_cards,
            value: dealer_value,
        },
        players,
        event_log: vec!["— snapshot —".into()],
    };

    // Seed log with current table state so history isn't blank on join
    for p in &state.players {
        if let Some(bet) = p.bet {
            state.event_log.push(format!("{} bet {}", p.name, bet));
        }
    }
    if !state.dealer.cards.is_empty() {
        state.event_log.push(format!(
            "dealer: {}",
            state
                .dealer
                .cards
                .iter()
                .map(|c| c.short_display())
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }

    state
}

fn apply_event_payload(app: &mut App, payload: bj_core::domain::engine::event::payload::EventPayload, seq: u64) {
    use bj_core::domain::engine::event::payload::EventPayload;
    use bj_core::domain::engine::phase::Phase;
    use crate::state::{
        cards::{UiCard, UiHand},
        table::{GamePhase, PlayerUiState},
    };

    // Extract phase change before borrowing screen
    let phase_change: Option<Phase> = match &payload {
        EventPayload::PhaseChanged { to, .. } => Some(to.clone()),
        _ => None,
    };

    // Apply payload to table state
    if let crate::state::Screen::Table(ref mut table) = app.ui.screen {
        table.event_seq = seq;

        match payload {
            EventPayload::PlayerJoined { player } => {
                let pid = player.to_string();
                if !table.players.iter().any(|p| p.player_id == pid) {
                    table.players.push(PlayerUiState {
                        player_id: pid.clone(),
                        name: short_id(&pid),
                        active: false,
                        hand: UiHand { cards: vec![], value: None },
                        hand_value: 0,
                        is_bust: false,
                        balance: 0,
                        bet: None,
                        status: "waiting".into(),
                    });
                }
                table.log(format!("#{seq} {} joined", short_id(&pid)));
            }
            EventPayload::PlayerLeft { player } => {
                let pid = player.to_string();
                table.log(format!("#{seq} {} left", short_id(&pid)));
                table.players.retain(|p| p.player_id != pid);
            }
            EventPayload::PlayerPlacedBet { player, amount } => {
                let pid = player.to_string();
                if let Some(p) = table.players.iter_mut().find(|p| p.player_id == pid) {
                    p.bet = Some(amount);
                    p.balance = p.balance.saturating_sub(amount);
                    p.status = "bet placed".into();
                }
                table.log(format!("#{seq} {} bet {}", short_id(&pid), amount));
            }
            EventPayload::GameStarted => {
                for p in &mut table.players {
                    p.hand.cards.clear();
                    p.hand.value = None;
                    p.hand_value = 0;
                    p.is_bust = false;
                    p.status = "playing".into();
                }
                table.dealer.cards.clear();
                table.dealer.value = None;
                table.phase = GamePhase::Dealing;
                table.log(format!("#{seq} — game started"));
            }
            EventPayload::PlayerCardDealt { player, card } => {
                let pid = player.to_string();
                let mut hand_value = 0u8;
                if let Some(p) = table.players.iter_mut().find(|p| p.player_id == pid) {
                    p.hand.cards.push(UiCard::visible(card));
                    p.hand_value = p.hand.compute_value();
                    p.hand.value = Some(p.hand_value.to_string());
                    hand_value = p.hand_value;
                }
                table.log(format!(
                    "#{seq} {} dealt {} (={})",
                    short_id(&pid),
                    UiCard::visible(card).short_display(),
                    hand_value
                ));
            }
            EventPayload::DealerCardDealt { card, .. } => {
                table.dealer.cards.push(UiCard::visible(card));
                let v = table.dealer.compute_value();
                table.dealer.value = if v > 0 { Some(v.to_string()) } else { None };
                table.log(format!("#{seq} dealer dealt {}", UiCard::visible(card).short_display()));
            }
            EventPayload::PlayerDecisionTaken { player, action } => {
                let pid = player.to_string();
                let action_str = format!("{:?}", action).to_lowercase();
                if let Some(p) = table.players.iter_mut().find(|p| p.player_id == pid) {
                    p.status = action_str.clone();
                }
                table.log(format!("#{seq} {} → {}", short_id(&pid), action_str));
            }
            EventPayload::PlayerBust { player } => {
                let pid = player.to_string();
                if let Some(p) = table.players.iter_mut().find(|p| p.player_id == pid) {
                    p.is_bust = true;
                    p.status = "BUST".into();
                }
                table.log(format!("#{seq} {} BUST", short_id(&pid)));
            }
            EventPayload::DealerBust { .. } => {
                table.log(format!("#{seq} dealer BUST"));
            }
            EventPayload::GameFinished { result } => {
                table.phase = GamePhase::Finished;
                for pr in &result.player_results {
                    let pid = pr.player.to_string();
                    let payout = pr.payout.total();
                    if let Some(p) = table.players.iter_mut().find(|p| p.player_id == pid) {
                        p.balance += payout;
                        p.bet = None;
                        p.status = format!("{:?} +{}", pr.outcome, payout);
                    }
                    table.log(format!(
                        "#{seq} {} {:?} payout:{}",
                        short_id(&pid),
                        pr.outcome,
                        payout
                    ));
                }
            }
            EventPayload::PhaseChanged { to, .. } => {
                let new_phase = server_phase_to_game_phase(&to);
                table.phase = new_phase;
                table.log(format!("#{seq} phase → {:?}", to));

                let active_pid = if let Phase::PlayerTurn(pid) = &to {
                    Some(pid.to_string())
                } else {
                    None
                };
                for p in &mut table.players {
                    p.active = active_pid.as_ref().map(|id| id == &p.player_id).unwrap_or(false);
                }

                // New round: reset cards and bets
                if matches!(to, Phase::WaitingForBets) {
                    for p in &mut table.players {
                        p.hand.cards.clear();
                        p.hand.value = None;
                        p.hand_value = 0;
                        p.is_bust = false;
                        p.bet = None;
                        p.status = "waiting".into();
                    }
                    table.dealer.cards.clear();
                    table.dealer.value = None;
                }
            }
        }
    }

    // After borrow on screen is dropped, sync UI chrome based on phase change
    if let Some(new_phase) = phase_change {
        sync_ui_chrome(app, server_phase_to_game_phase(&new_phase));
    }
}

fn sync_ui_chrome(app: &mut App, phase: crate::state::table::GamePhase) {
    use crate::state::{table::GamePhase, BettingState};
    use crate::state::ui_state::{FooterHint, FooterState};

    let min_bet = app.table_min_bet;
    let max_bet = app.table_max_bet;

    match phase {
        GamePhase::WaitingForBets | GamePhase::Betting => {
            app.ui.betting = Some(BettingState {
                min_bet: min_bet as u64,
                max_bet: max_bet as u64,
                current_bet: min_bet as u64,
                step: (min_bet as u64).max(5),
                confirmed: false,
            });
            app.ui.footer = FooterState {
                hints: vec![
                    FooterHint { key: "←→", label: "bet" },
                    FooterHint { key: "enter", label: "confirm" },
                    FooterHint { key: "l", label: "leave" },
                    FooterHint { key: "q", label: "quit" },
                ],
            };
            app.ui.header.subtitle = format!("Table – {}", phase);
        }
        GamePhase::PlayerTurn => {
            app.ui.betting = None;
            app.ui.footer = FooterState {
                hints: vec![
                    FooterHint { key: "h", label: "hit" },
                    FooterHint { key: "s", label: "stand" },
                    FooterHint { key: "l", label: "leave" },
                    FooterHint { key: "q", label: "quit" },
                ],
            };
            app.ui.header.subtitle = format!("Table – {}", phase);
        }
        _ => {
            app.ui.betting = None;
            app.ui.footer = FooterState {
                hints: vec![
                    FooterHint { key: "l", label: "leave" },
                    FooterHint { key: "q", label: "quit" },
                ],
            };
            app.ui.header.subtitle = format!("Table – {}", phase);
        }
    }
}

fn server_phase_to_game_phase(
    phase: &bj_core::domain::engine::phase::Phase,
) -> crate::state::table::GamePhase {
    use bj_core::domain::engine::phase::Phase;
    use crate::state::table::GamePhase;
    match phase {
        Phase::WaitingForBets => GamePhase::Betting,
        Phase::InitialDealing => GamePhase::Dealing,
        Phase::PlayerTurn(_) => GamePhase::PlayerTurn,
        Phase::DealerTurn => GamePhase::DealerTurn,
        Phase::Payouts => GamePhase::Resolving,
        Phase::Finished => GamePhase::Finished,
    }
}

fn short_id(id: &str) -> String {
    id.chars().take(8).collect()
}
