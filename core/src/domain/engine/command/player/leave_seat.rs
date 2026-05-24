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

        if state.waiting.contains(&self.player_id) {
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
            return Ok(vec![]);
        }

        Err(CommandError::PlayerNotFound(self.player_id))
    }
}
