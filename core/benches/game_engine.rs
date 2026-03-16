use bj_core::domain::{
    engine::{
        command::{
            CommandId, GameCommand,
            dealer::{DealerAction, DealerCommand, DealInitialCards, OpenBetting, PlayHand, SettleRound},
            player::{Hit, PlaceBet, PlayerAction, PlayerCommand, Stand},
            system::{CloseTable, PlayerTimeout, SystemCommand},
        },
        game_id::GameId,
        game_state::GameState,
        phase::Phase,
    },
    hand::Hand,
    Card, DeckId, DealerId, PlayerId, Rank, Shoe, Suit, TableSettings,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn pid(n: u64) -> PlayerId {
    PlayerId(n)
}

fn did() -> DealerId {
    DealerId(0)
}

fn gid() -> GameId {
    GameId::new()
}

fn cid() -> CommandId {
    CommandId(0)
}

fn default_settings() -> TableSettings {
    TableSettings {
        min_bet: 10,
        max_bet: 1000,
        max_players: 6,
        max_observers: 10,
    }
}

fn shoe_from_ranks(ranks: &[Rank]) -> Vec<Card> {
    ranks
        .iter()
        .map(|&rank| Card {
            deck_id: DeckId::One,
            suit: Suit::Spades,
            rank,
        })
        .collect()
}

fn place_bet_cmd(player_id: PlayerId, amount: u32) -> GameCommand {
    GameCommand::Player(PlayerCommand {
        game_id: gid(),
        command_id: cid(),
        action: PlayerAction::PlaceBet(PlaceBet { player_id, amount }),
    })
}

fn hit_cmd(player_id: PlayerId) -> GameCommand {
    GameCommand::Player(PlayerCommand {
        game_id: gid(),
        command_id: cid(),
        action: PlayerAction::Hit(Hit { player_id }),
    })
}

fn stand_cmd(player_id: PlayerId) -> GameCommand {
    GameCommand::Player(PlayerCommand {
        game_id: gid(),
        command_id: cid(),
        action: PlayerAction::Stand(Stand { player_id }),
    })
}

fn deal_initial_cards_cmd() -> GameCommand {
    GameCommand::Dealer(DealerCommand {
        game_id: gid(),
        command_id: cid(),
        action: DealerAction::DealInitialCards(DealInitialCards),
    })
}

fn play_hand_cmd() -> GameCommand {
    GameCommand::Dealer(DealerCommand {
        game_id: gid(),
        command_id: cid(),
        action: DealerAction::PlayHand(PlayHand),
    })
}

fn settle_round_cmd() -> GameCommand {
    GameCommand::Dealer(DealerCommand {
        game_id: gid(),
        command_id: cid(),
        action: DealerAction::SettleRound(SettleRound),
    })
}

/// Benchmark `Hand::value()` with various hands — does not require game logic.
fn bench_hand_value(c: &mut Criterion) {
    let make_card = |rank| Card {
        deck_id: DeckId::One,
        suit: Suit::Spades,
        rank,
    };

    c.bench_function("hand_value_hard_20", |b| {
        let mut hand = Hand::new();
        hand.add_card(make_card(Rank::King));
        hand.add_card(make_card(Rank::Queen));
        b.iter(|| black_box(hand.value()));
    });

    c.bench_function("hand_value_soft_18", |b| {
        let mut hand = Hand::new();
        hand.add_card(make_card(Rank::Ace));
        hand.add_card(make_card(Rank::Seven));
        b.iter(|| black_box(hand.value()));
    });

    c.bench_function("hand_value_blackjack", |b| {
        let mut hand = Hand::new();
        hand.add_card(make_card(Rank::Ace));
        hand.add_card(make_card(Rank::King));
        b.iter(|| black_box(hand.value()));
    });

    c.bench_function("hand_value_bust", |b| {
        let mut hand = Hand::new();
        hand.add_card(make_card(Rank::King));
        hand.add_card(make_card(Rank::Queen));
        hand.add_card(make_card(Rank::Five));
        b.iter(|| black_box(hand.value()));
    });

    c.bench_function("hand_value_5_card_hand", |b| {
        let mut hand = Hand::new();
        hand.add_card(make_card(Rank::Two));
        hand.add_card(make_card(Rank::Three));
        hand.add_card(make_card(Rank::Four));
        hand.add_card(make_card(Rank::Five));
        hand.add_card(make_card(Rank::Six));
        b.iter(|| black_box(hand.value()));
    });
}

/// Benchmark shoe shuffling.
fn bench_shoe_shuffle(c: &mut Criterion) {
    c.bench_function("shoe_shuffled", |b| {
        b.iter(|| black_box(Shoe::shuffled()));
    });
}

/// Benchmark GameState construction.
fn bench_game_state_new(c: &mut Criterion) {
    let shoe = Shoe::default().into_cards();
    let p1 = pid(1);
    let p2 = pid(2);

    c.bench_function("game_state_new_with_balance", |b| {
        b.iter(|| {
            black_box(GameState::new_with_balance(
                gid(),
                shoe.clone(),
                vec![(p1, 1000), (p2, 1000)],
                did(),
            ))
        });
    });
}

criterion_group!(benches, bench_hand_value, bench_shoe_shuffle, bench_game_state_new);
criterion_main!(benches);
