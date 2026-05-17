use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct OpenBetting;

impl CommandHandler for OpenBetting {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match &state.phase {
            Phase::WaitingForBets => Ok(vec![]),
            Phase::Finished => Ok(vec![EventPayload::PhaseChanged {
                from: Phase::Finished,
                to: Phase::WaitingForBets,
            }]),
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
        table::TableSettings,
        Shoe,
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
    fn open_betting_wrong_phase() {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.phase = Phase::DealerTurn;
        let err = GameEngine::handle(&state, &settings(), &cmd()).unwrap_err();
        assert!(matches!(err, CommandError::WrongPhase { .. }));
    }
}
