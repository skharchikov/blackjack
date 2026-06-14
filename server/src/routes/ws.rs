use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use tracing::info;

use crate::AppState;

#[utoipa::path(
    get,
    path = "/ws",
    responses(
        (status = 101, description = "WebSocket upgrade")
    )
)]
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let conn_id = state
        .connections
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    info!("WS connection {conn_id} opened");
    // Full protocol implemented in PR-08.
    let _ = socket.recv().await;
    info!("WS connection {conn_id} closed");
}
