use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bj_core::domain::TableSettings;
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

use crate::AppState;

// TODO(Task 16): Fully replace with new table/session architecture.
#[derive(Serialize, ToResponse, ToSchema)]
#[response(description = "Table information")]
struct TableResponse {
    id: String,
    name: String,
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
pub async fn list_tables(State(_state): State<AppState>) -> impl IntoResponse {
    // TODO(Task 16): Wire up real session store.
    let response: Vec<TableResponse> = vec![];
    (StatusCode::OK, Json(response))
}
