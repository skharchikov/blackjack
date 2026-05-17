use crate::domain::{
    engine::{
        command::CommandHandler, error::CommandError, event::payload::EventPayload,
        game_state::GameState,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct JoinTable {
    pub player_id: PlayerId,
}

impl CommandHandler for JoinTable {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        // Idempotent: already anywhere at this table
        if state.players.iter().any(|p| p.player_id == self.player_id)
            || state.observers.contains(&self.player_id)
            || state.waiting.contains(&self.player_id)
        {
            return Ok(vec![]);
        }
        Ok(vec![EventPayload::ObserverJoined {
            player: self.player_id,
        }])
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
            phase::Phase,
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

    fn empty_state() -> GameState {
        GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new())
    }

    fn cmd(player_id: PlayerId) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::JoinTable(JoinTable { player_id }),
        })
    }

    #[test]
    fn join_becomes_observer() {
        let state = empty_state();
        let pid = PlayerId::new();
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::ObserverJoined { player } if player == pid));
    }

    #[test]
    fn join_any_phase() {
        let mut state = empty_state();
        state.phase = Phase::DealerTurn;
        let pid = PlayerId::new();
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::ObserverJoined { .. }));
    }

    #[test]
    fn join_idempotent_as_observer() {
        let mut state = empty_state();
        let pid = PlayerId::new();
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        for e in &events {
            state.apply_event(e);
        }
        let events2 = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert!(events2.is_empty());
    }

    #[test]
    fn join_idempotent_as_seated_player() {
        let mut state = empty_state();
        let pid = PlayerId::new();
        // Manually seat the player
        state.players.push(crate::domain::player::PlayerState::new(pid));
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn join_idempotent_in_waiting_list() {
        let mut state = empty_state();
        let pid = PlayerId::new();
        state.waiting.push(pid);
        let events = GameEngine::handle(&state, &settings(5), &cmd(pid)).unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn join_full_table_allowed_as_observer() {
        let mut state = empty_state();
        let s = settings(1);
        let pid1 = PlayerId::new();
        state.players.push(crate::domain::player::PlayerState::new(pid1));
        // Table is full but join should still work (becomes observer)
        let pid2 = PlayerId::new();
        let events = GameEngine::handle(&state, &s, &cmd(pid2)).unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], EventPayload::ObserverJoined { .. }));
    }
}
