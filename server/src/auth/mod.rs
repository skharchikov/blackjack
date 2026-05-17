use async_trait::async_trait;
use bj_core::domain::PlayerId;
use thiserror::Error;
use ulid::Ulid;

#[derive(Debug)]
pub struct AuthPayload {
    pub player_id_str: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("authentication failed: {0}")]
    Failed(String),
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, payload: &AuthPayload) -> Result<PlayerId, AuthError>;
}

pub struct TrustedPlayerIdAuthenticator;

#[async_trait]
impl Authenticator for TrustedPlayerIdAuthenticator {
    async fn authenticate(&self, payload: &AuthPayload) -> Result<PlayerId, AuthError> {
        let ulid = payload
            .player_id_str
            .parse::<Ulid>()
            .unwrap_or_else(|_| Ulid::new());
        Ok(PlayerId(ulid))
    }
}
