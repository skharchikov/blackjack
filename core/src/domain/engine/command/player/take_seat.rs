use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct TakeSeat {
    pub player_id: PlayerId,
}

impl CommandHandler for TakeSeat {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        if !state.observers.contains(&self.player_id) {
            return Err(CommandError::PlayerNotFound(self.player_id));
        }

        if matches!(state.phase, Phase::WaitingForBets)
            && state.players.len() < settings.max_players
        {
            Ok(vec![EventPayload::PlayerJoined {
                player: self.player_id,
            }])
        } else {
            Ok(vec![EventPayload::PlayerAddedToWaitingList {
                player: self.player_id,
            }])
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
                player::{PlayerAction, PlayerCommand},
                CommandId, GameCommand,
            },
            game_id::GameId,
            game_state::GameState,
            GameEngine,
        },
        Shoe,
    };

    fn settings(max_players: usize) -> TableSettings {
        TableSettings {
            min_bet: 10,
            max_bet: 1000,
            max_players,
            max_observers: 10,
        }
    }

    fn state_with_observer(pid: PlayerId) -> GameState {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.observers.push(pid);
        state
    }

    fn cmd(player_id: PlayerId) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::TakeSeat(TakeSeat { player_id }),
        })
    }

    #[test]
    fn take_seat_waiting_for_bets_with_open_slot() {
        let pid = PlayerId::new();
        let state = state_with_observer(pid);
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::PlayerJoined { player } if player == pid));
    }

    #[test]
    fn take_seat_waiting_for_bets_table_full() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        let other = PlayerId::new();
        state
            .players
            .push(crate::domain::player::PlayerState::new(other));
        let events = GameEngine::handle(&state, &settings(1), &cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            events[0],
            EventPayload::PlayerAddedToWaitingList { player } if player == pid
        ));
    }

    #[test]
    fn take_seat_mid_game_goes_to_waiting() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        state.phase = Phase::PlayerTurn(PlayerId::new());
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            events[0],
            EventPayload::PlayerAddedToWaitingList { .. }
        ));
    }

    #[test]
    fn take_seat_not_observer_returns_error() {
        let pid = PlayerId::new();
        let state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let err = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap_err();
        assert!(matches!(err, CommandError::PlayerNotFound(_)));
    }

    #[test]
    fn take_seat_removes_from_observers_when_seated() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        for e in &events {
            state.apply_event(e);
        }
        assert!(!state.observers.contains(&pid));
        assert!(state.players.iter().any(|p| p.player_id == pid));
    }

    #[test]
    fn take_seat_removes_from_observers_when_waitlisted() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        state.phase = Phase::DealerTurn;
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        for e in &events {
            state.apply_event(e);
        }
        assert!(!state.observers.contains(&pid));
        assert!(state.waiting.contains(&pid));
    }
}
