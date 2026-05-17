use crate::domain::{
    engine::{
        action::PlayerDecision, command::CommandHandler, error::CommandError,
        event::payload::EventPayload, game_state::GameState, phase::Phase,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct Stand {
    pub player_id: PlayerId,
}

impl CommandHandler for Stand {
    fn handle(
        &self,
        state: &GameState,
        _settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        match &state.phase {
            Phase::PlayerTurn(id) if *id == self.player_id => {}
            _ => return Err(CommandError::NotPlayersTurn),
        }
        Ok(vec![
            EventPayload::PlayerDecisionTaken {
                player: self.player_id,
                action: PlayerDecision::Stand,
            },
            EventPayload::PhaseChanged {
                from: Phase::PlayerTurn(self.player_id),
                to: state.next_player_after(self.player_id),
            },
        ])
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

    fn stand_cmd(pid: PlayerId) -> GameCommand {
        GameCommand::Player(PlayerCommand {
            game_id: GameId::new(),
            command_id: CommandId(0),
            action: PlayerAction::Stand(Stand { player_id: pid }),
        })
    }

    fn state_in_player_turn(pid: PlayerId) -> GameState {
        let shoe = vec![card(Rank::King); 30];
        let mut state =
            GameState::new_with_balance(GameId::new(), shoe, vec![(pid, 1000)], DealerId::new());
        state.players[0].bet = Some(100);
        state.phase = Phase::PlayerTurn(pid);
        state
    }

    #[test]
    fn stand_single_player_goes_to_dealer() {
        let pid = PlayerId::new();
        let state = state_in_player_turn(pid);
        let events = GameEngine::handle(&state, &settings(), &stand_cmd(pid)).unwrap();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[1],
            EventPayload::PhaseChanged {
                to: Phase::DealerTurn,
                ..
            }
        ));
    }

    #[test]
    fn stand_advances_to_next_player() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let shoe = vec![card(Rank::King); 30];
        let mut state = GameState::new_with_balance(
            GameId::new(),
            shoe,
            vec![(p1, 1000), (p2, 1000)],
            DealerId::new(),
        );
        state.players[0].bet = Some(100);
        state.players[1].bet = Some(100);
        state.phase = Phase::PlayerTurn(p1);
        let events = GameEngine::handle(&state, &settings(), &stand_cmd(p1)).unwrap();
        assert!(
            matches!(events[1], EventPayload::PhaseChanged { to: Phase::PlayerTurn(id), .. } if id == p2)
        );
    }

    #[test]
    fn stand_wrong_turn() {
        let pid = PlayerId::new();
        let other = PlayerId::new();
        let state = state_in_player_turn(pid);
        assert!(matches!(
            GameEngine::handle(&state, &settings(), &stand_cmd(other)),
            Err(CommandError::NotPlayersTurn)
        ));
    }
}
