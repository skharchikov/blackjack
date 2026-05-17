use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{
    auth::{AuthPayload, Authenticator, Password},
    protocol::{ClientMessage, ServerMessage},
    session::RequestId,
    AppState,
};
use bj_core::domain::{
    engine::command::player::{Hit, JoinTable, LeaveTable, PlaceBet, PlayerAction, Stand},
    engine::snapshot::GameEventDto,
    TableId,
};

#[utoipa::path(get, path = "/ws", responses((status = 101, description = "WebSocket upgrade")))]
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let conn_id = state
        .connections
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    info!("WS connection {conn_id} opened");

    // Auth phase
    let player_id = loop {
        match socket.recv().await {
            None => return,
            Some(Err(e)) => {
                error!("recv error: {e}");
                return;
            }
            Some(Ok(Message::Text(text))) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::Auth { username, password }) => {
                        match state.auth
                            .authenticate(&AuthPayload {
                                username,
                                password: Password::new(password),
                            })
                            .await
                        {
                            Ok(pid) => {
                                // Only seed chips for new players (wallet returns Err if player unknown)
                                if state.wallet.balance(pid).await.is_err() {
                                    let _ = state.wallet.credit(pid, 1000).await;
                                }
                                let msg = ServerMessage::AuthOk {
                                    player_id: pid.to_string(),
                                };
                                if send_msg(&mut socket, &msg).await.is_err() {
                                    return;
                                }
                                break pid;
                            }
                            Err(e) => {
                                let _ = send_msg(
                                    &mut socket,
                                    &ServerMessage::AuthError {
                                        reason: e.to_string(),
                                    },
                                )
                                .await;
                                return;
                            }
                        }
                    }
                    _ => {
                        let _ = send_msg(
                            &mut socket,
                            &ServerMessage::AuthError {
                                reason: "send Auth first".into(),
                            },
                        )
                        .await;
                    }
                }
            }
            Some(Ok(_)) => {}
        }
    };

    let mut current_table: Option<TableId> = None;
    let (event_fwd_tx, mut event_fwd_rx) = mpsc::channel::<String>(64);
    let mut fwd_abort: Option<tokio::task::JoinHandle<()>> = None;

    loop {
        tokio::select! {
            Some(json) = event_fwd_rx.recv() => {
                if socket.send(Message::Text(json.into())).await.is_err() { break; }
            }
            msg = socket.recv() => {
                match msg {
                    None | Some(Err(_)) => break,
                    Some(Ok(Message::Close(_))) => break,
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Err(e) => {
                                let _ = send_msg(&mut socket, &ServerMessage::CommandError {
                                    request_id: 0,
                                    reason: format!("parse error: {e}"),
                                }).await;
                            }
                            Ok(msg) => {
                                if handle_client_msg(
                                    msg, player_id, &state,
                                    &mut socket, &mut current_table,
                                    &event_fwd_tx, &mut fwd_abort,
                                ).await.is_err() { break; }
                            }
                        }
                    }
                    Some(Ok(_)) => {}
                }
            }
        }
    }

    if let Some(h) = fwd_abort {
        h.abort();
    }
    if let Some(tid) = current_table {
        let _ = state
            .session
            .send_command(
                tid,
                player_id,
                RequestId(0),
                PlayerAction::LeaveTable(LeaveTable { player_id }),
            )
            .await;
    }
    info!("WS connection {conn_id} closed");
}

async fn handle_client_msg(
    msg: ClientMessage,
    player_id: bj_core::domain::PlayerId,
    state: &AppState,
    socket: &mut WebSocket,
    current_table: &mut Option<TableId>,
    event_fwd_tx: &mpsc::Sender<String>,
    fwd_abort: &mut Option<tokio::task::JoinHandle<()>>,
) -> Result<(), ()> {
    match msg {
        ClientMessage::JoinTable {
            table_id,
            request_id,
        } => {
            let tid = match table_id.parse::<TableId>() {
                Ok(t) => t,
                Err(_) => {
                    let _ = send_msg(
                        socket,
                        &ServerMessage::CommandError {
                            request_id,
                            reason: "invalid table_id".into(),
                        },
                    )
                    .await;
                    return Ok(());
                }
            };

            // Leave old table before joining a new one
            if let Some(old_tid) = current_table.take() {
                if let Some(h) = fwd_abort.take() {
                    h.abort();
                }
                let _ = state
                    .session
                    .send_command(
                        old_tid,
                        player_id,
                        RequestId(0),
                        PlayerAction::LeaveTable(LeaveTable { player_id }),
                    )
                    .await;
            }

            // Reject join if the table refuses it
            if let Err(e) = state
                .session
                .send_command(
                    tid,
                    player_id,
                    RequestId(request_id),
                    PlayerAction::JoinTable(JoinTable { player_id }),
                )
                .await
            {
                let _ = send_msg(
                    socket,
                    &ServerMessage::CommandError {
                        request_id,
                        reason: e.to_string(),
                    },
                )
                .await;
                return Ok(());
            }

            // Subscribe FIRST to avoid missing events between snapshot and subscribe
            let mut rx = match state.session.subscribe(tid).await {
                Ok(rx) => rx,
                Err(e) => {
                    let _ = send_msg(
                        socket,
                        &ServerMessage::CommandError {
                            request_id,
                            reason: e.to_string(),
                        },
                    )
                    .await;
                    return Ok(());
                }
            };

            // Then snapshot
            match state.session.snapshot(tid, player_id).await {
                Ok(snap) => {
                    let _ = send_msg(
                        socket,
                        &ServerMessage::Snapshot {
                            table_id: tid.to_string(),
                            state: snap,
                        },
                    )
                    .await;
                }
                Err(e) => {
                    let _ = send_msg(
                        socket,
                        &ServerMessage::CommandError {
                            request_id,
                            reason: e.to_string(),
                        },
                    )
                    .await;
                    return Ok(());
                }
            }

            // Start forwarder with the already-subscribed receiver
            if let Some(h) = fwd_abort.take() {
                h.abort();
            }
            let tx = event_fwd_tx.clone();
            let tid_str = tid.to_string();
            let handle = tokio::spawn(async move {
                loop {
                    match rx.recv().await {
                        Ok(event) => {
                            let dto = GameEventDto {
                                game_id: event.game_id,
                                seq: event.event_seq_id.0,
                                payload: event.payload,
                            };
                            let msg = ServerMessage::Event {
                                table_id: tid_str.clone(),
                                event: dto,
                            };
                            if let Ok(json) = serde_json::to_string(&msg) {
                                if tx.send(json).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                            let err = ServerMessage::CommandError {
                                request_id: 0,
                                reason: "event stream lagged, please rejoin".into(),
                            };
                            if let Ok(json) = serde_json::to_string(&err) {
                                let _ = tx.send(json).await;
                            }
                            break;
                        }
                        Err(_) => break,
                    }
                }
            });
            *fwd_abort = Some(handle);
            *current_table = Some(tid);
            let _ = send_msg(socket, &ServerMessage::CommandAck { request_id }).await;
        }

        ClientMessage::LeaveTable {
            table_id,
            request_id,
        } => {
            if let Ok(tid) = table_id.parse::<TableId>() {
                let _ = state
                    .session
                    .send_command(
                        tid,
                        player_id,
                        RequestId(request_id),
                        PlayerAction::LeaveTable(LeaveTable { player_id }),
                    )
                    .await;
                if let Some(h) = fwd_abort.take() {
                    h.abort();
                }
                *current_table = None;
                let _ = send_msg(socket, &ServerMessage::CommandAck { request_id }).await;
            }
        }

        ClientMessage::PlaceBet {
            table_id,
            request_id,
            amount,
        } => {
            send_player_cmd(
                socket,
                state,
                player_id,
                &table_id,
                request_id,
                PlayerAction::PlaceBet(PlaceBet { player_id, amount }),
            )
            .await?;
        }
        ClientMessage::Hit {
            table_id,
            request_id,
        } => {
            send_player_cmd(
                socket,
                state,
                player_id,
                &table_id,
                request_id,
                PlayerAction::Hit(Hit { player_id }),
            )
            .await?;
        }
        ClientMessage::Stand {
            table_id,
            request_id,
        } => {
            send_player_cmd(
                socket,
                state,
                player_id,
                &table_id,
                request_id,
                PlayerAction::Stand(Stand { player_id }),
            )
            .await?;
        }

        ClientMessage::DealerOpenBetting { request_id, .. }
        | ClientMessage::DealerDealCards { request_id, .. }
        | ClientMessage::DealerPlayHand { request_id, .. }
        | ClientMessage::DealerSettle { request_id, .. } => {
            let _ = send_msg(
                socket,
                &ServerMessage::CommandError {
                    request_id,
                    reason: "dealer commands not supported via WS in this version".into(),
                },
            )
            .await;
        }

        ClientMessage::Auth { .. } => {
            let _ = send_msg(
                socket,
                &ServerMessage::CommandError {
                    request_id: 0,
                    reason: "already authenticated".into(),
                },
            )
            .await;
        }
    }
    Ok(())
}

async fn send_player_cmd(
    socket: &mut WebSocket,
    state: &AppState,
    player_id: bj_core::domain::PlayerId,
    table_id_str: &str,
    request_id: u64,
    action: PlayerAction,
) -> Result<(), ()> {
    match table_id_str.parse::<TableId>() {
        Err(_) => {
            let _ = send_msg(
                socket,
                &ServerMessage::CommandError {
                    request_id,
                    reason: "invalid table_id".into(),
                },
            )
            .await;
        }
        Ok(tid) => {
            match state
                .session
                .send_command(tid, player_id, RequestId(request_id), action)
                .await
            {
                Ok(_) => {
                    let _ = send_msg(socket, &ServerMessage::CommandAck { request_id }).await;
                }
                Err(e) => {
                    let _ = send_msg(
                        socket,
                        &ServerMessage::CommandError {
                            request_id,
                            reason: e.to_string(),
                        },
                    )
                    .await;
                }
            }
        }
    }
    Ok(())
}

async fn send_msg(socket: &mut WebSocket, msg: &ServerMessage) -> Result<(), ()> {
    let json = serde_json::to_string(msg).map_err(|_| ())?;
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|_| ())
}
