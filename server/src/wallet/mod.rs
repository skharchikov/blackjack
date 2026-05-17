pub mod in_memory;

use async_trait::async_trait;
use bj_core::domain::PlayerId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("player not found")]
    PlayerNotFound,
    #[error("insufficient balance: have {balance}, need {amount}")]
    InsufficientBalance { balance: u32, amount: u32 },
}

#[async_trait]
pub trait Wallet: Send + Sync {
    async fn balance(&self, player: PlayerId) -> Result<u32, WalletError>;
    async fn debit(&self, player: PlayerId, amount: u32) -> Result<u32, WalletError>;
    async fn credit(&self, player: PlayerId, amount: u32) -> Result<u32, WalletError>;
}
