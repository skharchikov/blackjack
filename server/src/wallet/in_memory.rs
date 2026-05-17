use super::{Wallet, WalletError};
use async_trait::async_trait;
use bj_core::domain::PlayerId;
use dashmap::DashMap;
use std::sync::Arc;

pub struct InMemoryWallet {
    balances: Arc<DashMap<PlayerId, u32>>,
}

impl InMemoryWallet {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(DashMap::new()),
        }
    }

    pub fn seed(&self, player: PlayerId, amount: u32) {
        self.balances.insert(player, amount);
    }
}

impl Default for InMemoryWallet {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Wallet for InMemoryWallet {
    async fn balance(&self, player: PlayerId) -> Result<u32, WalletError> {
        self.balances
            .get(&player)
            .map(|b| *b)
            .ok_or(WalletError::PlayerNotFound)
    }

    async fn debit(&self, player: PlayerId, amount: u32) -> Result<u32, WalletError> {
        let mut entry = self
            .balances
            .get_mut(&player)
            .ok_or(WalletError::PlayerNotFound)?;
        if *entry < amount {
            return Err(WalletError::InsufficientBalance {
                balance: *entry,
                amount,
            });
        }
        *entry -= amount;
        Ok(*entry)
    }

    async fn credit(&self, player: PlayerId, amount: u32) -> Result<u32, WalletError> {
        let mut entry = self.balances.entry(player).or_insert(0);
        *entry += amount;
        Ok(*entry)
    }
}
