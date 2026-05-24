//! In-process load test: measures table / player / observer capacity.
//!
//! Bypasses the WS and HTTP layers to isolate `TableActor` and
//! `InMemoryGameSession` throughput.
//!
//! # Usage
//! ```
//! cargo run --example load_test -- [--tables N] [--players N] [--observers N] [--secs N]
//! ```
//! Defaults: 10 tables, 4 players/table, 8 observers/table, 30 s
//!
//! # What is measured
//! - Rounds completed per second (a round = one full Blackjack game)
//! - Commands sent per second (JoinTable + TakeSeat + PlaceBet + Stand per player per round)
//! - Events received per second by all observer tasks combined
//! - Command errors (rejected commands) — should be zero under correct play

use bj_core::domain::{
    engine::{
        command::player::{JoinTable, PlaceBet, PlayerAction, Stand, TakeSeat},
        event::payload::EventPayload,
        game_id::GameId,
        game_state::GameState,
        phase::Phase,
    },
    DealerId, PlayerId, Shoe, TableId, TableSettings,
};
use server::{
    session::{
        summary::TableSummary,
        table_actor::{run_table_actor_with_config, TableActorConfig, TableCommand},
        RequestId, SessionError,
    },
    wallet::in_memory::InMemoryWallet,
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc, RwLock};
use ulid::Ulid;

// ── metrics ──────────────────────────────────────────────────────────────────

#[derive(Default)]
struct Metrics {
    rounds_completed: AtomicU64,
    commands_sent: AtomicU64,
    commands_error: AtomicU64,
    events_received: AtomicU64,
    observer_lag: AtomicU64,
}

// ── table handle ──────────────────────────────────────────────────────────────

struct TableHandle {
    cmd_tx: mpsc::Sender<TableCommand>,
    event_tx: broadcast::Sender<bj_core::domain::engine::event::GameEvent>,
}

/// Spin up one `TableActor` with accelerated timers and return its handles.
fn create_table(
    wallet: Arc<dyn server::wallet::Wallet>,
    settings: TableSettings,
    betting_timeout: Duration,
    round_delay: Duration,
) -> TableHandle {
    let table_id = TableId::new();
    let dealer_id = DealerId(Ulid::new());
    let game_id = GameId::new();
    let shoe = Shoe::shuffled();
    let state = GameState::new(game_id, shoe, vec![], dealer_id);

    let (cmd_tx, cmd_rx) = mpsc::channel::<TableCommand>(512);
    let (event_tx, _) = broadcast::channel(1024);

    let summary = Arc::new(RwLock::new(TableSummary {
        id: table_id,
        name: "load-test".to_string(),
        settings: settings.clone(),
        player_count: 0,
        phase: "WaitingForBets".into(),
        is_joinable: true,
    }));

    let event_tx_clone = event_tx.clone();
    let summary_clone = summary.clone();
    let config = TableActorConfig {
        betting_timeout,
        round_delay,
        player_turn_timeout: Duration::from_secs(10),
    };

    tokio::spawn(run_table_actor_with_config(
        table_id,
        settings,
        state,
        cmd_rx,
        event_tx_clone,
        summary_clone,
        wallet,
        config,
    ));

    TableHandle { cmd_tx, event_tx }
}

// ── command helpers ───────────────────────────────────────────────────────────

async fn send_cmd(
    cmd_tx: &mpsc::Sender<TableCommand>,
    player_id: PlayerId,
    action: PlayerAction,
    req_id: u64,
    metrics: &Metrics,
) -> bool {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let cmd = TableCommand::Execute {
        player_id,
        request_id: RequestId(req_id),
        action,
        reply: tx,
    };
    metrics.commands_sent.fetch_add(1, Ordering::Relaxed);
    if cmd_tx.send(cmd).await.is_err() {
        metrics.commands_error.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    match rx.await {
        Ok(Ok(_)) => true,
        Ok(Err(SessionError::CommandRejected(_))) => {
            metrics.commands_error.fetch_add(1, Ordering::Relaxed);
            false
        }
        _ => {
            metrics.commands_error.fetch_add(1, Ordering::Relaxed);
            false
        }
    }
}

// ── player task ───────────────────────────────────────────────────────────────

/// Drives one player through repeated Blackjack rounds.
///
/// Flow per round:
/// 1. Try `PlaceBet(10)` immediately.
///    - On success → proceed to respond to turns.
///    - On failure (wrong phase / on waiting list) → wait for
///      `PhaseChanged{WaitingForBets}` then retry.
/// 2. After bet is placed, loop on events:
///    - `PhaseChanged { to: PlayerTurn(self) }` → send `Stand`
///    - `GameFinished`                          → count round, go to 1
async fn player_task(
    player_id: PlayerId,
    cmd_tx: mpsc::Sender<TableCommand>,
    event_tx: broadcast::Sender<bj_core::domain::engine::event::GameEvent>,
    wallet: Arc<dyn server::wallet::Wallet>,
    metrics: Arc<Metrics>,
    deadline: std::time::Instant,
) {
    wallet.set_balance(player_id, 1_000_000).await;

    let mut rx = event_tx.subscribe();
    let mut req = 1u64;

    // Join + take seat
    send_cmd(
        &cmd_tx,
        player_id,
        PlayerAction::JoinTable(JoinTable { player_id }),
        req,
        &metrics,
    )
    .await;
    req += 1;
    send_cmd(
        &cmd_tx,
        player_id,
        PlayerAction::TakeSeat(TakeSeat { player_id }),
        req,
        &metrics,
    )
    .await;
    req += 1;

    // Drain any events that arrived before our receiver was created
    while rx.try_recv().is_ok() {}

    // On the very first iteration the table is already in WaitingForBets, so we
    // try PlaceBet immediately.  On all subsequent iterations the phase will be
    // Finished (round_delay not elapsed yet), so we wait for the event first.
    let mut wait_for_betting_phase_first = false;

    'outer: loop {
        if std::time::Instant::now() >= deadline {
            break;
        }

        // ── betting phase ─────────────────────────────────────────────────────
        if wait_for_betting_phase_first {
            // After a completed round, wait for the actor to open the next one.
            if !wait_for_event(&mut rx, deadline, |e| {
                matches!(
                    e,
                    EventPayload::PhaseChanged {
                        to: Phase::WaitingForBets,
                        ..
                    }
                )
            })
            .await
            {
                break 'outer;
            }
        }

        // Try to place a bet.  If rejected (e.g. joined mid-round or on the
        // waiting list), wait for the next WaitingForBets event and retry once.
        let ok = send_cmd(
            &cmd_tx,
            player_id,
            PlayerAction::PlaceBet(PlaceBet {
                player_id,
                amount: 10,
            }),
            req,
            &metrics,
        )
        .await;
        req += 1;

        if !ok {
            // Unexpected rejection — wait for betting phase and retry
            if !wait_for_event(&mut rx, deadline, |e| {
                matches!(
                    e,
                    EventPayload::PhaseChanged {
                        to: Phase::WaitingForBets,
                        ..
                    }
                )
            })
            .await
            {
                break 'outer;
            }
            send_cmd(
                &cmd_tx,
                player_id,
                PlayerAction::PlaceBet(PlaceBet {
                    player_id,
                    amount: 10,
                }),
                req,
                &metrics,
            )
            .await;
            req += 1;
        }

        // ── play phase ────────────────────────────────────────────────────────
        // Respond to our turn (stand immediately).  Also handle `GameFinished`
        // which may arrive before our turn (e.g. blackjack auto-resolve).
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                break 'outer;
            }

            match tokio::time::timeout(remaining, rx.recv()).await {
                Ok(Ok(ev)) => match ev.payload {
                    EventPayload::PhaseChanged {
                        to: Phase::PlayerTurn(pid),
                        ..
                    } if pid == player_id => {
                        send_cmd(
                            &cmd_tx,
                            player_id,
                            PlayerAction::Stand(Stand { player_id }),
                            req,
                            &metrics,
                        )
                        .await;
                        req += 1;
                    }
                    EventPayload::GameFinished { .. } => {
                        metrics.rounds_completed.fetch_add(1, Ordering::Relaxed);
                        wait_for_betting_phase_first = true;
                        break; // back to betting phase
                    }
                    _ => {}
                },
                Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => {
                    // missed events — keep going
                }
                _ => break 'outer,
            }
        }
    }
}

/// Wait for an event matching `pred`, discarding non-matching events.
/// Returns `false` if deadline is reached before the event arrives.
async fn wait_for_event<F>(
    rx: &mut broadcast::Receiver<bj_core::domain::engine::event::GameEvent>,
    deadline: std::time::Instant,
    pred: F,
) -> bool
where
    F: Fn(&EventPayload) -> bool,
{
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return false;
        }
        match tokio::time::timeout(remaining, rx.recv()).await {
            Ok(Ok(ev)) => {
                if pred(&ev.payload) {
                    return true;
                }
            }
            Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => {
                // missed events — keep going
                continue;
            }
            _ => return false,
        }
    }
}

// ── observer task ─────────────────────────────────────────────────────────────

async fn observer_task(
    event_tx: broadcast::Sender<bj_core::domain::engine::event::GameEvent>,
    metrics: Arc<Metrics>,
    deadline: std::time::Instant,
) {
    let mut rx = event_tx.subscribe();
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match tokio::time::timeout(remaining, rx.recv()).await {
            Ok(Ok(_)) => {
                metrics.events_received.fetch_add(1, Ordering::Relaxed);
            }
            Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(n))) => {
                metrics.observer_lag.fetch_add(n, Ordering::Relaxed);
            }
            _ => break,
        }
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

fn parse_arg(args: &[String], flag: &str, default: usize) -> usize {
    args.windows(2)
        .find(|w| w[0] == flag)
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(default)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num_tables = parse_arg(&args, "--tables", 10);
    let players_per_table = parse_arg(&args, "--players", 4);
    let observers_per_table = parse_arg(&args, "--observers", 8);
    let duration_secs = parse_arg(&args, "--secs", 30);

    println!("╔═══════════════════════════════════════════╗");
    println!("║     Blackjack In-Process Load Test        ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║  Tables:           {:>8}              ║", num_tables);
    println!(
        "║  Players / table:  {:>8}              ║",
        players_per_table
    );
    println!(
        "║  Observers/table:  {:>8}              ║",
        observers_per_table
    );
    println!("║  Duration:         {:>7}s              ║", duration_secs);
    println!("╚═══════════════════════════════════════════╝");
    println!();
    println!("Spinning up {} tables…", num_tables);

    let metrics = Arc::new(Metrics::default());
    let wallet: Arc<dyn server::wallet::Wallet> = Arc::new(InMemoryWallet::new());

    let settings = TableSettings {
        min_bet: 10,
        max_bet: 100_000,
        max_players: players_per_table.max(1),
        max_observers: observers_per_table.max(1),
    };

    let deadline = std::time::Instant::now() + Duration::from_secs(duration_secs as u64);

    let mut task_handles = Vec::new();

    for _ in 0..num_tables {
        let handle = create_table(
            wallet.clone(),
            settings.clone(),
            Duration::from_secs(3),     // fast betting timeout
            Duration::from_millis(300), // fast round delay
        );

        // Spawn observer tasks
        for _ in 0..observers_per_table {
            let m = metrics.clone();
            let tx = handle.event_tx.clone();
            let d = deadline;
            task_handles.push(tokio::spawn(observer_task(tx, m, d)));
        }

        // Spawn player tasks
        for _ in 0..players_per_table {
            let player_id = PlayerId(Ulid::new());
            let m = metrics.clone();
            let cmd_tx = handle.cmd_tx.clone();
            let event_tx = handle.event_tx.clone();
            let w = wallet.clone();
            let d = deadline;
            task_handles.push(tokio::spawn(player_task(
                player_id, cmd_tx, event_tx, w, m, d,
            )));
        }
    }

    println!(
        "Running with {} total tasks for {}s…",
        task_handles.len(),
        duration_secs
    );

    // Progress ticker
    let m2 = metrics.clone();
    let ticker = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.tick().await; // skip first immediate tick
        let mut elapsed = 0u64;
        loop {
            interval.tick().await;
            elapsed += 5;
            let rounds = m2.rounds_completed.load(Ordering::Relaxed);
            let cmds = m2.commands_sent.load(Ordering::Relaxed);
            let evts = m2.events_received.load(Ordering::Relaxed);
            println!(
                "  [{:>3}s]  rounds={:>6}  cmds={:>8}  events={:>10}",
                elapsed, rounds, cmds, evts
            );
            if elapsed >= duration_secs as u64 {
                break;
            }
        }
    });

    // Wait for all tasks
    for h in task_handles {
        let _ = h.await;
    }
    ticker.abort();

    let elapsed = duration_secs as f64;
    let rounds = metrics.rounds_completed.load(Ordering::Relaxed);
    let cmds_sent = metrics.commands_sent.load(Ordering::Relaxed);
    let cmds_err = metrics.commands_error.load(Ordering::Relaxed);
    let events = metrics.events_received.load(Ordering::Relaxed);
    let lag = metrics.observer_lag.load(Ordering::Relaxed);

    println!();
    println!("╔═══════════════════════════════════════════╗");
    println!("║              Results                      ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║  Tables active:    {:>8}              ║", num_tables);
    println!(
        "║  Total players:    {:>8}              ║",
        num_tables * players_per_table
    );
    println!(
        "║  Total observers:  {:>8}              ║",
        num_tables * observers_per_table
    );
    println!("╠═══════════════════════════════════════════╣");
    println!(
        "║  Rounds completed: {:>8}  ({:.1}/s){}║",
        rounds,
        rounds as f64 / elapsed,
        pad(format!("{:.1}/s)", rounds as f64 / elapsed), 10)
    );
    println!(
        "║  Commands sent:    {:>8}  ({:.0}/s){}║",
        cmds_sent,
        cmds_sent as f64 / elapsed,
        pad(format!("{:.0}/s)", cmds_sent as f64 / elapsed), 10)
    );
    println!(
        "║  Command errors:   {:>8}  ({:.1}%){}║",
        cmds_err,
        if cmds_sent > 0 {
            100.0 * cmds_err as f64 / cmds_sent as f64
        } else {
            0.0
        },
        pad(
            format!(
                "{:.1}%)",
                if cmds_sent > 0 {
                    100.0 * cmds_err as f64 / cmds_sent as f64
                } else {
                    0.0
                }
            ),
            10
        )
    );
    println!(
        "║  Events received:  {:>8}  ({:.0}/s){}║",
        events,
        events as f64 / elapsed,
        pad(format!("{:.0}/s)", events as f64 / elapsed), 10)
    );
    if lag > 0 {
        println!("║  Observer lag:     {:>8}  (missed)       ║", lag);
    }
    println!("╚═══════════════════════════════════════════╝");
    println!();

    if cmds_err > 0 {
        println!(
            "⚠  {} command errors detected. Check game-flow logic in the load test.",
            cmds_err
        );
    }
    if lag > 0 {
        println!(
            "⚠  Observers missed {} events (broadcast channel lagged).",
            lag
        );
        println!(
            "   Increase the broadcast channel size (currently 1024) or reduce observer count."
        );
    }
    if cmds_err == 0 && lag == 0 {
        println!("✓  No errors or lag. System handled the load cleanly.");
    }
}

fn pad(s: String, width: usize) -> String {
    let spaces = width.saturating_sub(s.len());
    " ".repeat(spaces)
}
