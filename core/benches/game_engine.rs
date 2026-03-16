use bj_core::domain::{
    engine::{game_id::GameId, game_state::GameState},
    hand::Hand,
    Card, DealerId, DeckId, PlayerId, Rank, Shoe, Suit,
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

criterion_group!(
    benches,
    bench_hand_value,
    bench_shoe_shuffle,
    bench_game_state_new
);
criterion_main!(benches);
