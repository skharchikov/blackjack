use crate::{
    session::{summary::TableSummary, CommandAck, RequestId, SessionError},
    wallet::Wallet,
};
use bj_core::domain::{
    engine::{
        command::{
            dealer::{
                DealInitialCards, DealerAction, DealerCommand, OpenBetting, PlayHand, SettleRound,
            },
            player::{PlayerAction, PlayerCommand},
            system::{PlayerTimeout, SystemCommand},
            CommandId, GameCommand,
        },
        event::{EventSeqId, GameEvent},
        game_id::GameId,
        game_state::GameState,
        phase::Phase,
        snapshot::GameStateSnapshot,
        GameEngine,
    },
    PlayerId, Shoe, TableId, TableSettings,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::{broadcast, mpsc, oneshot, RwLock};
use tracing::{info, warn};

pub enum TableCommand {
    Execute {
        player_id: PlayerId,
        request_id: RequestId,
        action: PlayerAction,
        reply: oneshot::Sender<Result<CommandAck, SessionError>>,
    },
    #[allow(dead_code)]
    DealerExecute { action: DealerAction },
    Snapshot {
        requesting_player: PlayerId,
        reply: oneshot::Sender<Result<GameStateSnapshot, SessionError>>,
    },
}

pub async fn run_table_actor(
    _table_id: TableId,
    settings: TableSettings,
    initial_state: GameState,
    mut cmd_rx: mpsc::Receiver<TableCommand>,
    event_tx: broadcast::Sender<GameEvent>,
    summary: Arc<RwLock<TableSummary>>,
    wallet: Arc<dyn Wallet>,
) {
    let mut state = initial_state;
    let mut seq = 0u64;

    let betting_timeout = Duration::from_secs(30);
    let player_turn_timeout = Duration::from_secs(30);
    let round_delay = Duration::from_secs(5);

    let betting_dl = tokio::time::sleep(betting_timeout);
    tokio::pin!(betting_dl);

    let mut player_dl: Option<std::pin::Pin<Box<tokio::time::Sleep>>> = None;
    let mut round_dl: Option<std::pin::Pin<Box<tokio::time::Sleep>>> = None;

    loop {
        tokio::select! {
            cmd = cmd_rx.recv() => {
                let Some(cmd) = cmd else { break };
                match cmd {
                    TableCommand::Execute { player_id, request_id, action, reply } => {
                        let game_cmd = GameCommand::Player(PlayerCommand {
                            game_id: state.game_id,
                            command_id: CommandId(request_id.0),
                            action,
                        });
                        match GameEngine::handle(&state, &settings, &game_cmd) {
                            Err(e) => {
                                warn!("table={_table_id} player={player_id} command rejected: {e}");
                                let _ = reply.send(Err(SessionError::CommandRejected(e.to_string())));
                            }
                            Ok(events) => {
                                apply_and_broadcast(&mut state, &events, &event_tx, &mut seq);
                                // Load wallet balance for any player that just joined
                                for payload in &events {
                                    if let bj_core::domain::engine::event::payload::EventPayload::PlayerJoined { player } = payload {
                                        if let Ok(balance) = wallet.balance(*player).await {
                                            if let Some(ps) = state.players.iter_mut().find(|p| p.player_id == *player) {
                                                ps.balance = balance;
                                            }
                                        }
                                    }
                                }
                                update_summary(&summary, &state, &settings).await;
                                if matches!(state.phase, Phase::Finished) {
                                    handle_game_finished(&state, &wallet, &mut round_dl, round_delay).await;
                                }
                                reset_player_timer(&state, &mut player_dl, player_turn_timeout);
                                let _ = reply.send(Ok(CommandAck { request_id }));
                            }
                        }
                        maybe_advance_dealer(&mut state, &settings, &event_tx, &mut seq, &summary, &wallet, &mut round_dl, round_delay, &mut player_dl, player_turn_timeout).await;
                    }
                    TableCommand::DealerExecute { action } => {
                        let game_cmd = GameCommand::Dealer(DealerCommand {
                            game_id: state.game_id,
                            command_id: CommandId(0),
                            action,
                        });
                        match GameEngine::handle(&state, &settings, &game_cmd) {
                            Err(e) => warn!("table={_table_id} dealer command rejected: {e}"),
                            Ok(events) => {
                                apply_and_broadcast(&mut state, &events, &event_tx, &mut seq);
                                update_summary(&summary, &state, &settings).await;
                                if matches!(state.phase, Phase::Finished) {
                                    handle_game_finished(&state, &wallet, &mut round_dl, round_delay).await;
                                }
                                reset_player_timer(&state, &mut player_dl, player_turn_timeout);
                            }
                        }
                        maybe_advance_dealer(&mut state, &settings, &event_tx, &mut seq, &summary, &wallet, &mut round_dl, round_delay, &mut player_dl, player_turn_timeout).await;
                    }
                    TableCommand::Snapshot { requesting_player, reply } => {
                        let snap = GameStateSnapshot::from_state(&state, requesting_player);
                        let _ = reply.send(Ok(snap));
                    }
                }
            }

            // Betting phase timeout
            _ = &mut betting_dl, if matches!(state.phase, Phase::WaitingForBets) => {
                let has_bets = state.players.iter().any(|p| p.bet.is_some());
                if has_bets {
                    fire_dealer(&mut state, &settings, DealerAction::DealInitialCards(DealInitialCards), &event_tx, &mut seq, &summary, &wallet, &mut round_dl, round_delay, &mut player_dl, player_turn_timeout).await;
                }
                betting_dl.as_mut().reset(tokio::time::Instant::now() + betting_timeout);
            }

            // Player turn timeout
            _ = async {
                if let Some(ref mut t) = player_dl { t.as_mut().await }
                else { std::future::pending::<()>().await }
            }, if player_dl.is_some() && matches!(state.phase, Phase::PlayerTurn(_)) => {
                if let Phase::PlayerTurn(pid) = state.phase {
                    let cmd = GameCommand::System(SystemCommand::PlayerTimeout(PlayerTimeout { player_id: pid }));
                    if let Ok(events) = GameEngine::handle(&state, &settings, &cmd) {
                        apply_and_broadcast(&mut state, &events, &event_tx, &mut seq);
                        update_summary(&summary, &state, &settings).await;
                        maybe_advance_dealer(&mut state, &settings, &event_tx, &mut seq, &summary, &wallet, &mut round_dl, round_delay, &mut player_dl, player_turn_timeout).await;
                    }
                }
                reset_player_timer(&state, &mut player_dl, player_turn_timeout);
            }

            // New round delay
            _ = async {
                if let Some(ref mut t) = round_dl { t.as_mut().await }
                else { std::future::pending::<()>().await }
            }, if round_dl.is_some() => {
                round_dl = None;
                // Preserve player balances across rounds
                let players: Vec<(PlayerId, u32)> = state.players.iter()
                    .map(|p| (p.player_id, p.balance))
                    .collect();
                let dealer_id = state.dealer.dealer_id;
                let shoe = Shoe::shuffled();
                state = GameState::new_with_balance(GameId::new(), shoe, players, dealer_id);
                // Broadcast new round notification so subscribed clients know the round reset
                seq += 1;
                let _ = event_tx.send(GameEvent {
                    game_id: state.game_id,
                    event_seq_id: EventSeqId(seq),
                    payload: bj_core::domain::engine::event::payload::EventPayload::PhaseChanged {
                        from: bj_core::domain::engine::phase::Phase::Finished,
                        to: bj_core::domain::engine::phase::Phase::WaitingForBets,
                    },
                });
                fire_dealer(&mut state, &settings, DealerAction::OpenBetting(OpenBetting), &event_tx, &mut seq, &summary, &wallet, &mut round_dl, round_delay, &mut player_dl, player_turn_timeout).await;
                betting_dl.as_mut().reset(tokio::time::Instant::now() + betting_timeout);
            }
        }
    }
}

fn apply_and_broadcast(
    state: &mut GameState,
    events: &[bj_core::domain::engine::event::payload::EventPayload],
    tx: &broadcast::Sender<GameEvent>,
    seq: &mut u64,
) {
    for payload in events {
        state.apply_event(payload);
        *seq += 1;
        let _ = tx.send(GameEvent {
            game_id: state.game_id,
            event_seq_id: EventSeqId(*seq),
            payload: payload.clone(),
        });
    }
}

async fn update_summary(
    summary: &Arc<RwLock<TableSummary>>,
    state: &GameState,
    settings: &TableSettings,
) {
    let phase_str = match &state.phase {
        Phase::WaitingForBets => "WaitingForBets".to_string(),
        Phase::InitialDealing => "InitialDealing".to_string(),
        Phase::PlayerTurn(_) => "PlayerTurn".to_string(),
        Phase::DealerTurn => "DealerTurn".to_string(),
        Phase::Payouts => "Payouts".to_string(),
        Phase::Finished => "Finished".to_string(),
    };
    let player_count = state.players.len();
    let is_joinable = state.observers.len() < settings.max_observers;
    let mut s = summary.write().await;
    s.player_count = player_count;
    s.phase = phase_str;
    s.is_joinable = is_joinable;
}

async fn handle_game_finished(
    state: &GameState,
    wallet: &Arc<dyn Wallet>,
    round_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    delay: Duration,
) {
    for player in &state.players {
        wallet.set_balance(player.player_id, player.balance).await;
        info!(
            "game={} player={} balance synced to {}",
            state.game_id, player.player_id, player.balance
        );
    }
    *round_dl = Some(Box::pin(tokio::time::sleep(delay)));
}

fn reset_player_timer(
    state: &GameState,
    player_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    timeout: Duration,
) {
    if matches!(state.phase, Phase::PlayerTurn(_)) {
        *player_dl = Some(Box::pin(tokio::time::sleep(timeout)));
    } else {
        *player_dl = None;
    }
}

#[allow(clippy::too_many_arguments)]
async fn fire_dealer(
    state: &mut GameState,
    settings: &TableSettings,
    action: DealerAction,
    event_tx: &broadcast::Sender<GameEvent>,
    seq: &mut u64,
    summary: &Arc<RwLock<TableSummary>>,
    wallet: &Arc<dyn Wallet>,
    round_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    round_delay: Duration,
    player_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    player_timeout: Duration,
) {
    let cmd = GameCommand::Dealer(DealerCommand {
        game_id: state.game_id,
        command_id: CommandId(0),
        action,
    });
    if let Ok(events) = GameEngine::handle(state, settings, &cmd) {
        apply_and_broadcast(state, &events, event_tx, seq);
        update_summary(summary, state, settings).await;
        if matches!(state.phase, Phase::Finished) {
            handle_game_finished(state, wallet, round_dl, round_delay).await;
        }
        reset_player_timer(state, player_dl, player_timeout);
    }
}

#[allow(clippy::too_many_arguments)]
async fn maybe_advance_dealer(
    state: &mut GameState,
    settings: &TableSettings,
    event_tx: &broadcast::Sender<GameEvent>,
    seq: &mut u64,
    summary: &Arc<RwLock<TableSummary>>,
    wallet: &Arc<dyn Wallet>,
    round_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    round_delay: Duration,
    player_dl: &mut Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    player_timeout: Duration,
) {
    if matches!(state.phase, Phase::DealerTurn) {
        fire_dealer(
            state,
            settings,
            DealerAction::PlayHand(PlayHand),
            event_tx,
            seq,
            summary,
            wallet,
            round_dl,
            round_delay,
            player_dl,
            player_timeout,
        )
        .await;
    }
    if matches!(state.phase, Phase::Payouts) {
        fire_dealer(
            state,
            settings,
            DealerAction::SettleRound(SettleRound),
            event_tx,
            seq,
            summary,
            wallet,
            round_dl,
            round_delay,
            player_dl,
            player_timeout,
        )
        .await;
    }
}
