use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct LeaveTable {
    pub player_id: PlayerId,
}

impl CommandHandler for LeaveTable {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        if state.players.iter().any(|p| p.player_id == self.player_id) {
            let mut events = vec![EventPayload::PlayerLeft {
                player: self.player_id,
            }];

            if let Phase::PlayerTurn(active) = state.phase {
                if active == self.player_id {
                    let mut temp = state.clone();
                    temp.players.retain(|p| p.player_id != self.player_id);
                    let next = temp.next_player_after_leave();
                    events.push(EventPayload::PhaseChanged {
                        from: Phase::PlayerTurn(self.player_id),
                        to: next,
                    });
                }
            }

            return Ok(events);
        }

        if state.observers.contains(&self.player_id) {
            return Ok(vec![EventPayload::ObserverLeft {
                player: self.player_id,
            }]);
        }

        if state.waiting.iter().any(|(p, _)| *p == self.player_id) {
            return Ok(vec![EventPayload::PlayerRemovedFromWaitingList {
                player: self.player_id,
            }]);
        }

        Err(CommandError::PlayerNotFound(self.player_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dealer::DealerId,
        engine::{
            command::{
                player::{PlayerAction, PlayerCommand, TakeSeat},
                CommandId, GameCommand,
            },
            game_id::GameId,
            game_state::GameState,
            GameEngine,
        },
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

    /// Creates a state where the player is seated (in `players`).
    fn state_with_seated_player(pid: PlayerId) -> GameState {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        // Observer first, then take seat
        state.observers.push(pid);
        let events = GameEngine::handle(
            &state,
            &settings(),
            &GameCommand::Player(PlayerCommand {
                game_id: GameId::new(),
                command_id: CommandId(0),
                action: PlayerAction::TakeSeat(TakeSeat { player_id: pid, seat: Seat::One }),
            }),
        )
        .unwrap();
        for e in events {
            state.apply_event(&e);
        }
        state
    }

    /// Creates a state where the player is an observer.
    fn state_with_observer(pid: PlayerId) -> GameState {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.observers.push(pid);
        state
    }

    /// Creates a state where the player is in the waiting list.
    fn state_with_waiting_player(pid: PlayerId) -> GameState {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.waiting.push((pid, Seat::One));
        state
    }

    fn leave_cmd(pid: PlayerId) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::LeaveTable(LeaveTable { player_id: pid }),
        })
    }

    #[test]
    fn leave_seated_player() {
        let pid = PlayerId::new();
        let state = state_with_seated_player(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_cmd(pid)).unwrap();
        assert!(matches!(events[0], EventPayload::PlayerLeft { player } if player == pid));
    }

    #[test]
    fn leave_observer() {
        let pid = PlayerId::new();
        let state = state_with_observer(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::ObserverLeft { player } if player == pid));
    }

    #[test]
    fn leave_waiting_player() {
        let pid = PlayerId::new();
        let state = state_with_waiting_player(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(
            events[0],
            EventPayload::PlayerRemovedFromWaitingList { player } if player == pid
        ));
    }

    #[test]
    fn leave_unknown_player() {
        let state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let err = GameEngine::handle(&state, &settings(), &leave_cmd(PlayerId::new())).unwrap_err();
        assert!(matches!(err, CommandError::PlayerNotFound(_)));
    }

    #[test]
    fn leave_during_player_turn_advances_phase() {
        let pid = PlayerId::new();
        let mut state = state_with_seated_player(pid);
        state.phase = Phase::PlayerTurn(pid);
        state.players[0].bet = Some(10);
        let events = GameEngine::handle(&state, &settings(), &leave_cmd(pid)).unwrap();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[1],
            EventPayload::PhaseChanged {
                to: Phase::DealerTurn,
                ..
            }
        ));
    }
}
