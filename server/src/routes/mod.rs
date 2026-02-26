mod health;
mod table;
mod ws;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};

use crate::AppState;

#[derive(OpenApi)]
struct ApiDoc;

fn api_router() -> (Router<AppState>, utoipa::openapi::OpenApi) {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(utoipa_axum::routes!(health::health_check))
        .routes(utoipa_axum::routes!(table::list_tables))
        .routes(utoipa_axum::routes!(ws::ws_handler))
        .split_for_parts()
}

/// Build the OpenAPI spec with all routes registered.
pub fn openapi() -> utoipa::openapi::OpenApi {
    api_router().1
}

/// Create the application router with all routes.
pub fn create_router(state: AppState) -> Router {
    let (router, api) = api_router();

    router
        .merge(Redoc::with_url("/redoc", api))
        .with_state(state)
}
