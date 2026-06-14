use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    player::PlayerId,
    table::TableSettings,
    Seat,
};

#[derive(Debug, Clone)]
pub struct TakeSeat {
    pub player_id: PlayerId,
    pub seat: Seat,
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
        if self.seat.index() > settings.max_players {
            return Err(CommandError::SeatNotAvailable(self.seat, settings.max_players));
        }
        if state.players.iter().any(|p| p.seat == self.seat) {
            return Err(CommandError::SeatOccupied(self.seat));
        }

        if matches!(state.phase, Phase::WaitingForBets) {
            Ok(vec![EventPayload::PlayerJoined {
                player: self.player_id,
                seat: self.seat,
            }])
        } else {
            Ok(vec![EventPayload::PlayerAddedToWaitingList {
                player: self.player_id,
                seat: self.seat,
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
        Seat, Shoe,
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

    fn cmd(player_id: PlayerId, seat: Seat) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::TakeSeat(TakeSeat { player_id, seat }),
        })
    }

    #[test]
    fn take_seat_waiting_for_bets_with_open_slot() {
        let pid = PlayerId::new();
        let state = state_with_observer(pid);
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::One)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::PlayerJoined { player, seat: Seat::One } if player == pid));
    }

    #[test]
    fn take_seat_occupied_errors() {
        let pid = PlayerId::new();
        let other = PlayerId::new();
        let mut state = state_with_observer(pid);
        state.players.push(crate::domain::player::PlayerState::at_seat(other, Seat::One));
        let err = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::One)).unwrap_err();
        assert!(matches!(err, CommandError::SeatOccupied(Seat::One)));
    }

    #[test]
    fn take_seat_out_of_range_errors() {
        let pid = PlayerId::new();
        let state = state_with_observer(pid);
        let err = GameEngine::handle(&state, &settings(2), &cmd(pid, Seat::Three)).unwrap_err();
        assert!(matches!(err, CommandError::SeatNotAvailable(Seat::Three, 2)));
    }

    #[test]
    fn take_seat_mid_game_goes_to_waiting() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        state.phase = Phase::PlayerTurn(PlayerId::new());
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::Two)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            events[0],
            EventPayload::PlayerAddedToWaitingList { seat: Seat::Two, .. }
        ));
    }

    #[test]
    fn take_seat_not_observer_returns_error() {
        let pid = PlayerId::new();
        let state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let err = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::One)).unwrap_err();
        assert!(matches!(err, CommandError::PlayerNotFound(_)));
    }

    #[test]
    fn take_seat_removes_from_observers_when_seated() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::One)).unwrap();
        for e in &events {
            state.apply_event(e);
        }
        assert!(!state.observers.contains(&pid));
        assert!(state.players.iter().any(|p| p.player_id == pid && p.seat == Seat::One));
    }

    #[test]
    fn take_seat_removes_from_observers_when_waitlisted() {
        let pid = PlayerId::new();
        let mut state = state_with_observer(pid);
        state.phase = Phase::DealerTurn;
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid, Seat::Three)).unwrap();
        for e in &events {
            state.apply_event(e);
        }
        assert!(!state.observers.contains(&pid));
        assert!(state.waiting.iter().any(|(p, s)| *p == pid && *s == Seat::Three));
    }
}
