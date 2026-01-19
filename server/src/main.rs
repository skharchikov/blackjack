use server::{routes::create_router, App, AppState};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state: AppState = Arc::new(RwLock::new(App::default()));
    let app = create_router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://localhost:3000");
    info!("WebSocket endpoint: ws://localhost:3000/ws");

    axum::serve(listener, app).await.unwrap();
}
