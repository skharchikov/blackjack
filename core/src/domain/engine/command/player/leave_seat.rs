use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState, phase::Phase,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct LeaveSeat {
    pub player_id: PlayerId,
}

impl CommandHandler for LeaveSeat {
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
            events.push(EventPayload::ObserverJoined {
                player: self.player_id,
            });
            return Ok(events);
        }

        if state.waiting.iter().any(|(p, _)| *p == self.player_id) {
            return Ok(vec![
                EventPayload::PlayerRemovedFromWaitingList {
                    player: self.player_id,
                },
                EventPayload::ObserverJoined {
                    player: self.player_id,
                },
            ]);
        }

        if state.observers.contains(&self.player_id) {
            // Already an observer — idempotent no-op, not an error.
            return Ok(vec![]);
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
                player::{LeaveSeat, PlayerAction, PlayerCommand, TakeSeat},
                CommandId, GameCommand,
            },
            game_id::GameId,
            game_state::GameState,
            phase::Phase,
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

    fn leave_seat_cmd(pid: PlayerId) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::LeaveSeat(LeaveSeat { player_id: pid }),
        })
    }

    fn state_with_seated_player(pid: PlayerId) -> GameState {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
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

    #[test]
    fn seated_player_emits_left_and_observer_joined() {
        let pid = PlayerId::new();
        let state = state_with_seated_player(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_seat_cmd(pid)).unwrap();
        assert!(
            matches!(events.first(), Some(EventPayload::PlayerLeft { player }) if *player == pid)
        );
        assert!(
            matches!(events.last(), Some(EventPayload::ObserverJoined { player }) if *player == pid)
        );
    }

    #[test]
    fn mid_round_phase_advances_when_active_player_leaves_seat() {
        let pid = PlayerId::new();
        let mut state = state_with_seated_player(pid);
        state.phase = Phase::PlayerTurn(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_seat_cmd(pid)).unwrap();
        let has_phase_change = events
            .iter()
            .any(|e| matches!(e, EventPayload::PhaseChanged { from: Phase::PlayerTurn(p), .. } if *p == pid));
        assert!(
            has_phase_change,
            "expected PhaseChanged when active player leaves seat"
        );
    }

    #[test]
    fn waiting_list_player_returns_to_observer() {
        let pid = PlayerId::new();
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.waiting.push((pid, Seat::One));
        let events = GameEngine::handle(&state, &settings(), &leave_seat_cmd(pid)).unwrap();
        assert!(
            matches!(events[0], EventPayload::PlayerRemovedFromWaitingList { player } if player == pid)
        );
        assert!(matches!(events[1], EventPayload::ObserverJoined { player } if player == pid));
    }

    #[test]
    fn observer_leave_seat_is_noop() {
        let pid = PlayerId::new();
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        state.observers.push(pid);
        let events = GameEngine::handle(&state, &settings(), &leave_seat_cmd(pid)).unwrap();
        assert!(
            events.is_empty(),
            "already an observer — should be idempotent no-op"
        );
    }

    #[test]
    fn unknown_player_returns_error() {
        let pid = PlayerId::new();
        let state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let err = GameEngine::handle(&state, &settings(), &leave_seat_cmd(pid)).unwrap_err();
        assert!(matches!(err, CommandError::PlayerNotFound(p) if p == pid));
    }
}
