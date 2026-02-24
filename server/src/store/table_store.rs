use blackjack_core::domain::Table;
use color_eyre::eyre::{eyre, Report};
use sqlx::PgPool;
use thiserror::Error;

#[async_trait::async_trait]
pub trait TableStore: Send + Sync {
    async fn list_tables(&self) -> Result<Vec<Table>, TableStoreError>;
}

#[derive(Debug, Error)]
pub enum TableStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

pub struct PostgresTableStore {
    pool: PgPool,
}

impl PostgresTableStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableStore for PostgresTableStore {
    async fn list_tables(&self) -> Result<Vec<Table>, TableStoreError> {
        let tables = sqlx::query_as::<_, Table>(r#"SELECT id, name, status, settings FROM tables"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TableStoreError::UnexpectedError(eyre!(e)))?;

        Ok(tables)
    }
}
