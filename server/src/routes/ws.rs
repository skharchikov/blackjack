use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::AppState;

// TODO(Task 17): Replace with proper ClientMessage/ServerMessage wire types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerMessage {
    Pong,
}

#[utoipa::path(
    get,
    path = "/ws",
    responses(
        (status = 101, description = "WebSocket upgrade")
    )
)]
pub async fn ws_handler(ws: WebSocketUpgrade, State(_state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

/// Handle an individual WebSocket connection.
async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket connection established");

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
            Message::Text(_) => {}
            Message::Ping(data) => {
                if let Err(e) = socket.send(Message::Pong(data)).await {
                    error!("Failed to send pong frame: {}", e);
                    break;
                }
            }
            Message::Close(_) => {
                info!("WebSocket connection closed");
                break;
            }
            _ => {}
        }
    }

    info!("WebSocket connection terminated");
}

/// Send a server message over the WebSocket.
async fn send_message(socket: &mut WebSocket, msg: &ServerMessage) -> Result<(), String> {
    let json = serde_json::to_string(msg).map_err(|e| e.to_string())?;
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| e.to_string())
}
