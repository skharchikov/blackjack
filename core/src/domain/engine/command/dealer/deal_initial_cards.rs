use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct DealInitialCards;

impl CommandHandler for DealInitialCards {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        if !matches!(state.phase, Phase::WaitingForBets) {
            return Err(CommandError::WrongPhase {
                actual: state.phase.clone(),
            });
        }
        let bettors: Vec<_> = state.players.iter().filter(|p| p.bet.is_some()).collect();
        if bettors.is_empty() {
            return Err(CommandError::NoBettors);
        }
        let needed = 2 * (bettors.len() + 1);
        if state.cards_remaining() < needed {
            return Err(CommandError::ShoeEmpty);
        }

        let mut events = vec![EventPayload::GameStarted];
        let mut idx = state.dealt;

        // Round 1: one card to each player, then dealer
        for p in &bettors {
            events.push(EventPayload::PlayerCardDealt {
                player: p.player_id,
                card: state.shoe[idx],
            });
            idx += 1;
        }
        events.push(EventPayload::DealerCardDealt {
            dealer: state.dealer.dealer_id,
            card: state.shoe[idx],
        });
        idx += 1;

        // Round 2: second card to each player, then dealer
        for p in &bettors {
            events.push(EventPayload::PlayerCardDealt {
                player: p.player_id,
                card: state.shoe[idx],
            });
            idx += 1;
        }
        // Second dealer card is the hole card — do not reveal in the event.
        events.push(EventPayload::DealerHoleCardDealt {
            dealer: state.dealer.dealer_id,
        });

        let first = bettors[0].player_id;
        events.push(EventPayload::PhaseChanged {
            from: Phase::InitialDealing,
            to: Phase::PlayerTurn(first),
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
            command::{
                dealer::{DealerAction, DealerCommand},
                CommandId, GameCommand,
            },
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
            action: DealerAction::DealInitialCards(DealInitialCards),
        })
    }

    fn state_with_bet(pid: PlayerId) -> GameState {
        let shoe = vec![
            card(Rank::King),
            card(Rank::Seven), // round 1: player, dealer
            card(Rank::Ace),
            card(Rank::Three), // round 2: player, dealer
        ];
        let mut state =
            GameState::new_with_balance(GameId::new(), shoe, vec![(pid, 1000)], DealerId::new());
        state.players[0].bet = Some(100);
        state
    }

    #[test]
    fn deals_correct_event_count_one_player() {
        let pid = PlayerId::new();
        let state = state_with_bet(pid);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        // GameStarted + 2 PlayerCardDealt + DealerCardDealt + DealerHoleCardDealt + PhaseChanged = 6
        assert_eq!(events.len(), 6);
    }

    #[test]
    fn first_event_is_game_started() {
        let pid = PlayerId::new();
        let state = state_with_bet(pid);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert!(matches!(events[0], EventPayload::GameStarted));
    }

    #[test]
    fn last_event_advances_to_player_turn() {
        let pid = PlayerId::new();
        let state = state_with_bet(pid);
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert!(matches!(
            events.last().unwrap(),
            EventPayload::PhaseChanged {
                to: Phase::PlayerTurn(_),
                ..
            }
        ));
    }

    #[test]
    fn no_bettors_errors() {
        let state = GameState::new(
            GameId::new(),
            crate::domain::Shoe::shuffled(),
            vec![],
            DealerId::new(),
        );
        assert!(matches!(
            GameEngine::handle(&state, &settings(), &cmd()),
            Err(CommandError::NoBettors)
        ));
    }

    #[test]
    fn wrong_phase_errors() {
        let pid = PlayerId::new();
        let mut state = state_with_bet(pid);
        state.phase = Phase::DealerTurn;
        assert!(matches!(
            GameEngine::handle(&state, &settings(), &cmd()),
            Err(CommandError::WrongPhase { .. })
        ));
    }
}
