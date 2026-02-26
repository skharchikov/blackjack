use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
