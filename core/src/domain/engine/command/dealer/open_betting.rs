use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    table::TableSettings,
    Seat,
};

#[derive(Debug, Clone)]
pub struct OpenBetting;

impl CommandHandler for OpenBetting {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match &state.phase {
            Phase::WaitingForBets => Ok(vec![]),
            Phase::Finished => {
                let mut events = vec![];
                let mut occupied: std::collections::BTreeSet<Seat> =
                    state.players.iter().map(|p| p.seat).collect();
                let available_seats = settings.max_players.saturating_sub(state.players.len());
                for &(pid, desired) in state.waiting.iter().take(available_seats) {
                    let seat = if !occupied.contains(&desired) {
                        desired
                    } else {
                        Seat::ALL
                            .iter()
                            .take(settings.max_players)
                            .copied()
                            .find(|s| !occupied.contains(s))
                            .expect("seat available — available_seats guard ensures capacity")
                    };
                    occupied.insert(seat);
                    events.push(EventPayload::PlayerJoined { player: pid, seat });
                }
                events.push(EventPayload::PhaseChanged {
                    from: Phase::Finished,
                    to: Phase::WaitingForBets,
                });
                Ok(events)
            }
            actual => Err(CommandError::WrongPhase {
                actual: actual.clone(),
            }),
        }
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
            GameEngine,
        },
        player::PlayerId,
        table::TableSettings,
        Seat, Shoe,
    };

    fn settings() -> TableSettings {
        TableSettings {
            min_bet: 10,
            max_bet: 1000,
            max_players: 5,
            max_observers: 10,
        }
    }

    fn cmd() -> GameCommand {
        GameCommand::Dealer(DealerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: DealerAction::OpenBetting(OpenBetting),
        })
    }

    #[test]
    fn open_betting_noop_when_already_waiting() {
        let state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn open_betting_from_finished() {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.phase = Phase::Finished;
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            EventPayload::PhaseChanged {
                to: Phase::WaitingForBets,
                ..
            }
        ));
    }

    #[test]
    fn open_betting_promotes_waiting_players() {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.phase = Phase::Finished;
        let pid1 = PlayerId::new();
        let pid2 = PlayerId::new();
        state.waiting.push((pid1, Seat::One));
        state.waiting.push((pid2, Seat::Two));
        let events = GameEngine::handle(&state, &settings(), &cmd()).unwrap();
        // Two PlayerJoined + one PhaseChanged
        assert_eq!(events.len(), 3);
        assert!(matches!(events[0], EventPayload::PlayerJoined { player, seat: Seat::One } if player == pid1));
        assert!(matches!(events[1], EventPayload::PlayerJoined { player, seat: Seat::Two } if player == pid2));
        assert!(matches!(
            &events[2],
            EventPayload::PhaseChanged {
                to: Phase::WaitingForBets,
                ..
            }
        ));
    }

    #[test]
    fn open_betting_respects_max_players() {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.phase = Phase::Finished;
        // Fill seat One
        state
            .players
            .push(crate::domain::player::PlayerState::at_seat(PlayerId::new(), Seat::One));
        // Two waiting but max_players is 2 so only one slot left
        let pid1 = PlayerId::new();
        let pid2 = PlayerId::new();
        state.waiting.push((pid1, Seat::Two));
        state.waiting.push((pid2, Seat::Three));

        let s = TableSettings {
            min_bet: 10,
            max_bet: 1000,
            max_players: 2,
            max_observers: 10,
        };
        let events = GameEngine::handle(&state, &s, &cmd()).unwrap();
        // One PlayerJoined + PhaseChanged
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], EventPayload::PlayerJoined { player, .. } if player == pid1));
    }

    #[test]
    fn open_betting_wrong_phase() {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.phase = Phase::DealerTurn;
        let err = GameEngine::handle(&state, &settings(), &cmd()).unwrap_err();
        assert!(matches!(err, CommandError::WrongPhase { .. }));
    }
}
