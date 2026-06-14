mod password;

pub use password::Password;

use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use bj_core::domain::PlayerId;
use thiserror::Error;

#[derive(Debug)]
pub struct AuthPayload {
    pub username: String,
    pub password: Password,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("wrong password")]
    WrongPassword,
    #[error("authentication failed: {0}")]
    Failed(String),
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Returns the PlayerId on success. Auto-creates the user on first login (signup).
    async fn authenticate(&self, payload: &AuthPayload) -> Result<PlayerId, AuthError>;
}

struct UserRecord {
    password: Password,
    player_id: PlayerId,
}

pub struct InMemoryAuthenticator {
    users: RwLock<HashMap<String, UserRecord>>,
    /// Reverse index: PlayerId → username, for O(1) display-name lookup.
    reverse: RwLock<HashMap<PlayerId, String>>,
}

impl Default for InMemoryAuthenticator {
    fn default() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            reverse: RwLock::new(HashMap::new()),
        }
    }
}

impl InMemoryAuthenticator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-register a user and return their PlayerId. Idempotent: returns existing id if username taken.
    pub fn seed_user(&self, username: &str, password: &str) -> PlayerId {
        let mut users = self.users.write().unwrap();
        if let Some(record) = users.get(username) {
            return record.player_id;
        }
        let pid = PlayerId::new();
        users.insert(
            username.to_string(),
            UserRecord {
                password: Password::new(password.to_string()),
                player_id: pid,
            },
        );
        drop(users);
        self.reverse
            .write()
            .unwrap()
            .insert(pid, username.to_string());
        pid
    }

    /// Resolve a PlayerId to the username used at login.
    pub fn lookup_username(&self, player_id: PlayerId) -> Option<String> {
        self.reverse.read().unwrap().get(&player_id).cloned()
    }
}

#[async_trait]
impl Authenticator for InMemoryAuthenticator {
    async fn authenticate(&self, payload: &AuthPayload) -> Result<PlayerId, AuthError> {
        let mut users = self.users.write().unwrap();
        if let Some(record) = users.get(&payload.username) {
            if record.password == payload.password {
                let pid = record.player_id;
                return Ok(pid);
            } else {
                return Err(AuthError::WrongPassword);
            }
        }
        // Auto-register on first login
        let pid = PlayerId::new();
        users.insert(
            payload.username.clone(),
            UserRecord {
                password: payload.password.clone(),
                player_id: pid,
            },
        );
        drop(users);
        self.reverse
            .write()
            .unwrap()
            .insert(pid, payload.username.clone());
        Ok(pid)
    }
}
