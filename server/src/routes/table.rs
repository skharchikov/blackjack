use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use blackjack_core::domain::{TableSettings, TableStatus};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct TableResponse {
    id: String,
    name: String,
    status: TableStatus,
    settings: TableSettings,
}

pub async fn list_tables(State(state): State<AppState>) -> impl IntoResponse {
    match state.table_store.list_tables().await {
        Ok(tables) => {
            let response: Vec<TableResponse> = tables
                .into_iter()
                .map(|t| TableResponse {
                    id: t.id.0.to_string(),
                    name: t.name,
                    status: t.status,
                    settings: t.settings,
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Failed to list tables: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
