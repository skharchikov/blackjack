mod password;

pub use password::Password;

use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use bj_core::domain::PlayerId;
use thiserror::Error;
use ulid::Ulid;

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
}

impl InMemoryAuthenticator {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }

    /// Pre-register a user and return their PlayerId. Idempotent: returns existing id if username taken.
    pub fn seed_user(&self, username: &str, password: &str) -> PlayerId {
        let mut users = self.users.write().unwrap();
        if let Some(record) = users.get(username) {
            return record.player_id;
        }
        let player_id = PlayerId(Ulid::new());
        users.insert(
            username.to_string(),
            UserRecord {
                password: Password::new(password.to_string()),
                player_id,
            },
        );
        player_id
    }
}

#[async_trait]
impl Authenticator for InMemoryAuthenticator {
    async fn authenticate(&self, payload: &AuthPayload) -> Result<PlayerId, AuthError> {
        // Fast path: read lock
        {
            let users = self.users.read().unwrap();
            if let Some(record) = users.get(&payload.username) {
                return if record.password == payload.password {
                    Ok(record.player_id)
                } else {
                    Err(AuthError::WrongPassword)
                };
            }
        }
        // User not found — create account (signup)
        let player_id = PlayerId(Ulid::new());
        self.users.write().unwrap().insert(
            payload.username.clone(),
            UserRecord {
                password: payload.password.clone(),
                player_id,
            },
        );
        Ok(player_id)
    }
}
