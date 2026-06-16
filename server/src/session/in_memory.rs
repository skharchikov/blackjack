use crate::{
    session::{
        summary::TableSummary,
        table_actor::{run_table_actor, TableCommand},
        CommandAck, GameSession, RequestId, SessionError,
    },
    wallet::Wallet,
};
use async_trait::async_trait;
use bj_core::domain::{
    engine::{
        command::player::PlayerAction, event::GameEvent, game_id::GameId, game_state::GameState,
        snapshot::GameStateSnapshot,
    },
    DealerId, PlayerId, Shoe, TableId, TableSettings,
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use ulid::Ulid;

struct TableHandle {
    cmd_tx: mpsc::Sender<TableCommand>,
    event_tx: broadcast::Sender<GameEvent>,
    summary: Arc<RwLock<TableSummary>>,
}

pub struct InMemoryGameSession {
    tables: DashMap<TableId, TableHandle>,
    wallet: Arc<dyn Wallet>,
}

struct SeedTable {
    name: &'static str,
    settings: TableSettings,
}

fn seeds() -> Vec<SeedTable> {
    vec![
        SeedTable {
            name: "Cool Kids #1",
            settings: TableSettings {
                min_bet: 10,
                max_bet: 500,
                max_players: 5,
                max_observers: 10,
            },
        },
        SeedTable {
            name: "Big Sharks #2",
            settings: TableSettings {
                min_bet: 25,
                max_bet: 1000,
                max_players: 6,
                max_observers: 10,
            },
        },
        SeedTable {
            name: "Sopranos #3",
            settings: TableSettings {
                min_bet: 100,
                max_bet: 5000,
                max_players: 4,
                max_observers: 10,
            },
        },
    ]
}

impl InMemoryGameSession {
    pub fn new(wallet: Arc<dyn Wallet>) -> Arc<Self> {
        let session = Arc::new(Self {
            tables: DashMap::new(),
            wallet,
        });
        for seed in seeds() {
            session.seed_table(seed.name, seed.settings);
        }
        session
    }

    fn seed_table(&self, name: &str, settings: TableSettings) {
        let table_id = TableId::new();
        let dealer_id = DealerId(Ulid::new());
        let game_id = GameId::new();
        let shoe = Shoe::shuffled();
        let state = GameState::new(game_id, shoe, vec![], dealer_id);

        let (cmd_tx, cmd_rx) = mpsc::channel::<TableCommand>(128);
        let (event_tx, _) = broadcast::channel::<GameEvent>(256);
        let summary = Arc::new(RwLock::new(TableSummary {
            id: table_id,
            name: name.to_string(),
            settings: settings.clone(),
            player_count: 0,
            phase: "WaitingForBets".into(),
            is_joinable: true,
        }));

        let wallet = self.wallet.clone();
        let summary_clone = summary.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(run_table_actor(
            table_id,
            settings,
            state,
            cmd_rx,
            event_tx_clone,
            summary_clone,
            wallet,
        ));

        self.tables.insert(
            table_id,
            TableHandle {
                cmd_tx,
                event_tx,
                summary,
            },
        );
    }
}

#[async_trait]
impl GameSession for InMemoryGameSession {
    async fn list_tables(&self) -> Vec<TableSummary> {
        // Collect Arc handles first, then drop the DashMap guard before awaiting
        let summaries: Vec<Arc<RwLock<TableSummary>>> = self
            .tables
            .iter()
            .map(|r| r.value().summary.clone())
            .collect();
        let mut out = Vec::with_capacity(summaries.len());
        for s in summaries {
            out.push(s.read().await.clone());
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    async fn snapshot(
        &self,
        table_id: TableId,
        player: PlayerId,
    ) -> Result<GameStateSnapshot, SessionError> {
        // Clone the sender out of the guard before awaiting
        let cmd_tx = self
            .tables
            .get(&table_id)
            .ok_or(SessionError::TableNotFound)?
            .cmd_tx
            .clone();
        let (tx, rx) = tokio::sync::oneshot::channel();
        cmd_tx
            .send(TableCommand::Snapshot {
                requesting_player: player,
                reply: tx,
            })
            .await
            .map_err(|_| SessionError::Internal)?;
        rx.await.map_err(|_| SessionError::Internal)?
    }

    async fn send_command(
        &self,
        table_id: TableId,
        player_id: PlayerId,
        request_id: RequestId,
        action: PlayerAction,
    ) -> Result<CommandAck, SessionError> {
        // Clone the sender out of the guard before awaiting
        let cmd_tx = self
            .tables
            .get(&table_id)
            .ok_or(SessionError::TableNotFound)?
            .cmd_tx
            .clone();
        let (tx, rx) = tokio::sync::oneshot::channel();
        cmd_tx
            .send(TableCommand::Execute {
                player_id,
                request_id,
                action,
                reply: tx,
            })
            .await
            .map_err(|_| SessionError::Internal)?;
        rx.await.map_err(|_| SessionError::Internal)?
    }

    async fn subscribe(
        &self,
        table_id: TableId,
    ) -> Result<broadcast::Receiver<GameEvent>, SessionError> {
        // Clone the sender out of the guard before awaiting
        let event_tx = self
            .tables
            .get(&table_id)
            .ok_or(SessionError::TableNotFound)?
            .event_tx
            .clone();
        Ok(event_tx.subscribe())
    }
}
