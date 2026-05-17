use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct PlayHand;

impl CommandHandler for PlayHand {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        if !matches!(state.phase, Phase::DealerTurn) {
            return Err(CommandError::WrongPhase {
                actual: state.phase.clone(),
            });
        }

        let mut events = vec![];

        // Reveal the hole card to clients before the dealer plays.
        if state.dealer.hand.cards.len() >= 2 {
            events.push(EventPayload::DealerHoleCardRevealed {
                dealer: state.dealer.dealer_id,
                card: state.dealer.hand.cards[1],
            });
        }

        let mut hand = state.dealer.hand.clone();
        let mut dealt = state.dealt;

        loop {
            let score = hand.value().best_value();
            if score >= 17 {
                break;
            }
            let card = *state.shoe.get(dealt).ok_or(CommandError::ShoeEmpty)?;
            events.push(EventPayload::DealerCardDealt {
                dealer: state.dealer.dealer_id,
                card,
            });
            hand.add_card(card);
            dealt += 1;
        }

        if hand.value().is_bust() {
            events.push(EventPayload::DealerBust {
                dealer: state.dealer.dealer_id,
            });
        }
        events.push(EventPayload::PhaseChanged {
            from: Phase::DealerTurn,
            to: Phase::Payouts,
        });

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dealer::DealerId,
        engine::{
            command::{CommandId, DealerCommand, GameCommand},
            game_id::GameId,
            game_state::GameState,
            GameEngine,
        },
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
        use crate::domain::engine::command::DealerAction;
        GameCommand::Dealer(DealerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: DealerAction::PlayHand(PlayHand),
        })
    }

    fn state_with_dealer_hand(dealer_cards: Vec<Rank>, shoe_cards: Vec<Rank>) -> GameState {
        let shoe: Vec<Card> = shoe_cards.iter().map(|&r| card(r)).collect();
        let mut state = GameState::new(GameId::new(), shoe, vec![], DealerId::new());
        state.phase = Phase::DealerTurn;
        for r in dealer_cards {
            state.dealer.hand.add_card(card(r));
        }
        state
    }

    #[test]
    fn dealer_stands_at_17() {
        // Dealer has King(10)+Seven(7)=17, no more cards needed
        let state = state_with_dealer_hand(vec![Rank::King, Rank::Seven], vec![Rank::Two; 10]);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        // DealerHoleCardRevealed + PhaseChanged
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], EventPayload::DealerHoleCardRevealed { .. }));
        assert!(matches!(
            events[1],
            EventPayload::PhaseChanged {
                to: Phase::Payouts,
                ..
            }
        ));
    }

    #[test]
    fn dealer_draws_to_17() {
        // Dealer has King(10)+Five(5)=15, draws Two(2) -> 17
        let state = state_with_dealer_hand(vec![Rank::King, Rank::Five], vec![Rank::Two]);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        // DealerHoleCardRevealed + DealerCardDealt + PhaseChanged
        assert_eq!(events.len(), 3);
        assert!(matches!(events[0], EventPayload::DealerHoleCardRevealed { .. }));
        assert!(matches!(events[1], EventPayload::DealerCardDealt { .. }));
    }

    #[test]
    fn dealer_busts() {
        // Dealer has King(10)+Six(6)=16, draws King(10) -> 26 bust
        let state = state_with_dealer_hand(vec![Rank::King, Rank::Six], vec![Rank::King]);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        // DealerHoleCardRevealed + DealerCardDealt + DealerBust + PhaseChanged
        assert_eq!(events.len(), 4);
        assert!(matches!(events[0], EventPayload::DealerHoleCardRevealed { .. }));
        assert!(matches!(events[2], EventPayload::DealerBust { .. }));
    }

    #[test]
    fn wrong_phase_errors() {
        let state = GameState::new(
            GameId::new(),
            crate::domain::Shoe::shuffled(),
            vec![],
            DealerId::new(),
        );
        // default phase is WaitingForBets
        assert!(matches!(
            GameEngine::handle(&state, &settings(), &cmd()),
            Err(CommandError::WrongPhase { .. })
        ));
    }
}
