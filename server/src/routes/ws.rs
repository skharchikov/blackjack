use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use tracing::{error, info};

use crate::{AppState, ServerMessage};

/// WebSocket upgrade handler.
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle an individual WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let conn_id = state
        .connections
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;

    info!("Connection {} established", conn_id);

    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        };

        match msg {
            Message::Text(text) if text.eq_ignore_ascii_case("ping") => {
                if let Err(e) = send_message(&mut socket, &ServerMessage::Pong).await {
                    error!("Failed to send pong: {}", e);
                    break;
                }
            }
            Message::Text(_) => {
                // Ignore other text messages
            }
            Message::Ping(data) => {
                if let Err(e) = socket.send(Message::Pong(data)).await {
                    error!("Failed to send pong frame: {}", e);
                    break;
                }
            }
            Message::Close(_) => {
                info!("Connection {} closed", conn_id);
                break;
            }
            _ => {}
        }
    }

    info!("Connection {} terminated", conn_id);
}

/// Send a server message over the WebSocket.
async fn send_message(socket: &mut WebSocket, msg: &ServerMessage) -> Result<(), String> {
    let json = serde_json::to_string(msg).map_err(|e| e.to_string())?;
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| e.to_string())
}
