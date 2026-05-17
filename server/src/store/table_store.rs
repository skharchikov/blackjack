use color_eyre::eyre::Report;
use sqlx::PgPool;
use thiserror::Error;

// TODO(Task 15): Replace with TableActor / InMemoryGameSession — this is a temporary stub.
#[async_trait::async_trait]
pub trait TableStore: Send + Sync {
    async fn list_tables(&self) -> Result<Vec<()>, TableStoreError>;
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
    async fn list_tables(&self) -> Result<Vec<()>, TableStoreError> {
        let _ = &self.pool;
        todo!("replaced in Task 15")
    }
}
