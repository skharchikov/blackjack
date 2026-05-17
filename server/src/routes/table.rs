use axum::{extract::State, Json};
use crate::{AppState, session::summary::TableSummary};

#[utoipa::path(
    get,
    path = "/tables",
    responses(
        (status = 200, description = "List of live tables")
    )
)]
pub async fn list_tables(State(state): State<AppState>) -> Json<Vec<TableSummary>> {
    Json(state.session.list_tables().await)
}
