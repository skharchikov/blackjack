pub mod auth;
pub mod config;
pub mod protocol;
pub mod routes;
pub mod session;
pub mod store;
pub mod wallet;

use auth::InMemoryAuthenticator;
use session::GameSession;
use std::sync::Arc;
use wallet::Wallet;

pub type AppState = Arc<App>;

pub struct App {
    pub session: Arc<dyn GameSession>,
    pub wallet: Arc<dyn Wallet>,
    pub auth: Arc<InMemoryAuthenticator>,
}

impl App {
    pub fn new(
        session: Arc<dyn GameSession>,
        wallet: Arc<dyn Wallet>,
        auth: Arc<InMemoryAuthenticator>,
    ) -> Self {
        Self {
            session,
            wallet,
            auth,
        }
    }
}
