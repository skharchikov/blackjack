use crate::domain::{
    engine::{
        command::{player::Stand, CommandHandler},
        error::CommandError,
        event::payload::EventPayload,
        game_state::GameState,
    },
    player::PlayerId,
    table::TableSettings,
};

#[derive(Debug, Clone)]
pub struct PlayerTimeout {
    pub player_id: PlayerId,
}

impl CommandHandler for PlayerTimeout {
    fn handle(
        &self,
        state: &GameState,
        settings: &TableSettings,
    ) -> Result<Vec<EventPayload>, CommandError> {
        Stand {
            player_id: self.player_id,
        }
        .handle(state, settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dealer::DealerId,
        engine::{
            command::{system::SystemCommand, GameCommand},
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

    #[test]
    fn timeout_acts_as_stand() {
        let pid = PlayerId::new();
        let shoe = vec![card(Rank::King); 30];
        let mut state =
            GameState::new_with_balance(GameId::new(), shoe, vec![(pid, 1000)], DealerId::new());
        state.players[0].bet = Some(100);
        state.phase = Phase::PlayerTurn(pid);

        let cmd = GameCommand::System(SystemCommand::PlayerTimeout(PlayerTimeout {
            player_id: pid,
        }));
        let events = GameEngine::handle(&state, &settings(), &cmd).unwrap();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[1],
            EventPayload::PhaseChanged {
                to: Phase::DealerTurn,
                ..
            }
        ));
    }
}
