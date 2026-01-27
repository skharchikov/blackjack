use server::config::Settings;
use server::{routes::create_router, App, AppState};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Settings::load().expect("Failed to load configuration");
    info!("Loaded configuration: {:?}", config);
    let state: AppState = Arc::new(RwLock::new(App::default()));
    let app = create_router(state);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("failed to bind to address");
    info!("Server running on http://localhost:3000");
    info!("WebSocket endpoint: ws://localhost:3000/ws");

    axum::serve(listener, app)
        .await
        .expect("failed to run Blackjack server");
}
