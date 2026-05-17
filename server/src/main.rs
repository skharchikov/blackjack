use server::auth::InMemoryAuthenticator;
use server::config::Settings;
use server::session::in_memory::InMemoryGameSession;
use server::wallet::in_memory::InMemoryWallet;
use server::{routes::create_router, App, AppState};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SEED_ACCOUNTS: &[(&str, &str)] = &[
    ("admin", "famly1234"),
    ("qa", "famly1234"),
    ("dev", "famly1234"),
];
const SEED_BALANCE: u32 = 1000;

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

    let wallet = Arc::new(InMemoryWallet::new());
    let auth = Arc::new(InMemoryAuthenticator::new());

    for (username, password) in SEED_ACCOUNTS {
        let pid = auth.seed_user(username, password);
        wallet.seed(pid, SEED_BALANCE);
        info!("Seeded account '{}' with {} chips", username, SEED_BALANCE);
    }

    let wallet_dyn: Arc<dyn server::wallet::Wallet> = wallet;
    let session = InMemoryGameSession::new(wallet_dyn.clone());
    let session: Arc<dyn server::session::GameSession> = session;

    let state: AppState = Arc::new(App::new(session, wallet_dyn, auth));
    let app = create_router(state);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("failed to bind to address");

    info!(
        "Server running on http://{}:{}",
        config.application.host, config.application.port
    );
    axum::serve(listener, app)
        .await
        .expect("failed to run server");
}
