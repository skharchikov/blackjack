pub mod config;
pub mod routes;
pub mod session;
pub mod store;
pub mod wallet;

use session::GameSession;
use std::sync::Arc;
use wallet::Wallet;

pub type AppState = Arc<App>;

pub struct App {
    pub session: Arc<dyn GameSession>,
    pub wallet: Arc<dyn Wallet>,
}

impl App {
    pub fn new(session: Arc<dyn GameSession>, wallet: Arc<dyn Wallet>) -> Self {
        Self { session, wallet }
    }
}
