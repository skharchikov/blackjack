use server::config::Settings;
use server::session::in_memory::InMemoryGameSession;
use server::wallet::in_memory::InMemoryWallet;
use server::{routes::create_router, App, AppState};
use std::sync::Arc;
use tokio::net::TcpListener;
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

    let config = Settings::load().expect("Failed to load configuration");
    info!("Loaded configuration: {:?}", config);

    let wallet: Arc<dyn server::wallet::Wallet> = Arc::new(InMemoryWallet::new());
    let session = InMemoryGameSession::new(wallet.clone());
    let session: Arc<dyn server::session::GameSession> = session;

    let state: AppState = Arc::new(App::new(session, wallet));
    let app = create_router(state);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("failed to bind to address");

    info!("Server running on http://{}:{}", config.application.host, config.application.port);
    axum::serve(listener, app).await.expect("failed to run server");
}
