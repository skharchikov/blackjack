use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bj_core::domain::{TableSettings, TableStatus};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

use crate::AppState;

#[derive(Serialize, ToResponse, ToSchema)]
#[response(description = "Table information")]
struct TableResponse {
    id: String,
    name: String,
    status: TableStatus,
    settings: TableSettings,
}

#[utoipa::path(
    get,
    path = "/tables",
    responses(
        (status = 200, description = "List of tables", body = [TableResponse]),
        (status = 500, description = "Internal server error")
    )
)]
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
