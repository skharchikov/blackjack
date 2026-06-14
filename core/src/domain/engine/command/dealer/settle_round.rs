use crate::domain::{
    engine::{
        command::CommandHandler,
        error::CommandError,
        event::{
            outcome::{GameResult, Payout, PayoutMultiplier, PlayerOutcome, PlayerResult},
            payload::EventPayload,
        },
        game_state::GameState,
        phase::Phase,
    },
    hand::Hand,
    player::PlayerState,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct SettleRound;

fn is_natural_blackjack(hand: &Hand) -> bool {
    hand.cards.len() == 2 && hand.value().best_value() == 21
}

fn settle_player(
    player: &PlayerState,
    dealer_hand: &Hand,
    dealer_busted: bool,
) -> (PlayerOutcome, PayoutMultiplier) {
    if player.hand.value().is_bust() {
        return (PlayerOutcome::Bust, PayoutMultiplier::Loss);
    }
    let player_bj = is_natural_blackjack(&player.hand);
    let dealer_bj = is_natural_blackjack(dealer_hand);

    if player_bj && dealer_bj {
        return (PlayerOutcome::Push, PayoutMultiplier::Push);
    }
    if player_bj {
        return (PlayerOutcome::Blackjack, PayoutMultiplier::Blackjack);
    }
    if dealer_bj {
        return (PlayerOutcome::Lost, PayoutMultiplier::Loss);
    }
    if dealer_busted {
        return (PlayerOutcome::Won, PayoutMultiplier::Win);
    }
    let pv = player.hand.value().best_value();
    let dv = dealer_hand.value().best_value();
    if pv > dv {
        (PlayerOutcome::Won, PayoutMultiplier::Win)
    } else if pv == dv {
        (PlayerOutcome::Push, PayoutMultiplier::Push)
    } else {
        (PlayerOutcome::Lost, PayoutMultiplier::Loss)
    }
}

impl CommandHandler for SettleRound {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        if !matches!(state.phase, Phase::Payouts) {
            return Err(CommandError::WrongPhase {
                actual: state.phase.clone(),
            });
        }
        let dealer_busted = state.dealer.hand.value().is_bust();
        let player_results: Vec<PlayerResult> = state
            .players
            .iter()
            .filter(|p| p.bet.is_some())
            .map(|p| {
                let (outcome, multiplier) = settle_player(p, &state.dealer.hand, dealer_busted);
                let bet = p.bet.unwrap();
                PlayerResult {
                    player: p.player_id,
                    outcome,
                    payout: Payout::new(bet, multiplier),
                }
            })
            .collect();

        Ok(vec![
            EventPayload::GameFinished {
                result: GameResult {
                    player_results,
                    dealer_busted,
                },
            },
            EventPayload::PhaseChanged {
                from: Phase::Payouts,
                to: Phase::Finished,
            },
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dealer::DealerId,
        engine::{
            command::{
                dealer::{DealerAction, DealerCommand},
                CommandId, GameCommand,
            },
            event::outcome::PlayerOutcome,
            game_id::GameId,
            game_state::GameState,
            GameEngine,
        },
        player::PlayerId,
        table::TableSettings,
        Card, DeckId, Rank, Suit,
    };

    fn settings() -> TableSettings {
        TableSettings {
            min_bet: 10,
            max_bet: 500,
            max_players: 5,
            max_observers: 10,
        }
    }

    fn card(rank: Rank) -> Card {
        Card {
            deck_id: DeckId::One,
            rank,
            suit: Suit::Spades,
        }
    }

    fn cmd() -> GameCommand {
        GameCommand::Dealer(DealerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: DealerAction::SettleRound(SettleRound),
        })
    }

    fn state_at_payouts(
        player_ranks: Vec<Rank>,
        dealer_ranks: Vec<Rank>,
        bet: u32,
    ) -> (GameState, PlayerId) {
        let pid = PlayerId::new();
        let shoe = crate::domain::Shoe::shuffled();
        let mut state =
            GameState::new_with_balance(GameId::new(), shoe, vec![(pid, 1000)], DealerId::new());
        state.phase = Phase::Payouts;
        state.players[0].bet = Some(bet);
        for r in player_ranks {
            state.players[0].hand.add_card(card(r));
        }
        for r in dealer_ranks {
            state.dealer.hand.add_card(card(r));
        }
        (state, pid)
    }

    fn outcome_for(events: &[EventPayload], pid: PlayerId) -> PlayerOutcome {
        for e in events {
            if let EventPayload::GameFinished { result } = e {
                for r in &result.player_results {
                    if r.player == pid {
                        return r.outcome.clone();
                    }
                }
            }
        }
        panic!("no result found")
    }

    #[test]
    fn player_wins() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Nine],
            vec![Rank::King, Rank::Seven],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Won);
    }

    #[test]
    fn player_loses() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Seven],
            vec![Rank::King, Rank::Nine],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Lost);
    }

    #[test]
    fn push() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Eight],
            vec![Rank::King, Rank::Eight],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Push);
    }

    #[test]
    fn player_blackjack() {
        let (state, pid) = state_at_payouts(
            vec![Rank::Ace, Rank::King],
            vec![Rank::King, Rank::Eight],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Blackjack);
    }

    #[test]
    fn both_blackjack_push() {
        let (state, pid) = state_at_payouts(
            vec![Rank::Ace, Rank::King],
            vec![Rank::Ace, Rank::King],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Push);
    }

    #[test]
    fn dealer_busts_player_wins() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Eight],
            vec![Rank::King, Rank::Queen, Rank::Five],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Won);
    }

    #[test]
    fn player_busts_loses() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Queen, Rank::Five],
            vec![Rank::King, Rank::Eight],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Bust);
    }

    #[test]
    fn wrong_phase() {
        let state = GameState::new(
            GameId::new(),
            crate::domain::Shoe::shuffled(),
            vec![],
            DealerId::new(),
        );
        assert!(matches!(
            GameEngine::handle(&state, &settings(), &cmd()),
            Err(CommandError::WrongPhase { .. })
        ));
    }

    #[test]
    fn dealer_blackjack_beats_player_21() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Seven, Rank::Four], // non-natural 21
            vec![Rank::Ace, Rank::King],               // dealer blackjack
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(outcome_for(&events, pid), PlayerOutcome::Lost);
    }

    #[test]
    fn payout_total_on_win() {
        let (state, pid) = state_at_payouts(
            vec![Rank::King, Rank::Nine],
            vec![Rank::King, Rank::Seven],
            100,
        );
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        if let EventPayload::GameFinished { result } = &events[0] {
            let r = result
                .player_results
                .iter()
                .find(|r| r.player == pid)
                .unwrap();
            assert_eq!(r.payout.total(), 200);
        }
    }
}
