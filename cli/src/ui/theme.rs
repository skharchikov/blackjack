use ratatui::style::Color;

use crate::state::table::RoundOutcome;

// Tokyo Night based semantic palette. Widgets use roles, never raw colors.
pub const ACCENT: Color = Color::Rgb(224, 175, 104);
pub const INFO: Color = Color::Rgb(125, 207, 255);
pub const WIN: Color = Color::Rgb(158, 206, 106);
pub const LOSE: Color = Color::Rgb(247, 118, 142);
pub const PUSH: Color = Color::Rgb(255, 158, 100);
pub const BLACKJACK: Color = Color::Rgb(187, 154, 247);
pub const TEXT: Color = Color::Rgb(192, 202, 245);
pub const MUTED: Color = Color::Rgb(86, 95, 137);
pub const SUBTLE: Color = Color::Rgb(60, 67, 100);
pub const BG_POPUP: Color = Color::Rgb(26, 27, 38);

// Table
pub const FELT: Color = Color::Rgb(20, 60, 40);
pub const FELT_TEXT: Color = Color::Rgb(170, 150, 90);
pub const RAIL: Color = Color::Rgb(140, 100, 55);

// Cards
pub const CARD_FACE: Color = Color::Rgb(232, 226, 210);
pub const CARD_INK: Color = Color::Rgb(30, 30, 40);
pub const CARD_RED: Color = Color::Rgb(200, 40, 50);
pub const CARD_BACK_FG: Color = Color::Rgb(122, 162, 247);
pub const CARD_BACK_BG: Color = Color::Rgb(40, 50, 90);

pub fn outcome_color(outcome: &RoundOutcome) -> Color {
    match outcome {
        RoundOutcome::Blackjack => BLACKJACK,
        RoundOutcome::Won => WIN,
        RoundOutcome::Push => PUSH,
        RoundOutcome::Lost | RoundOutcome::Bust => LOSE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_colors() {
        assert_eq!(outcome_color(&RoundOutcome::Blackjack), BLACKJACK);
        assert_eq!(outcome_color(&RoundOutcome::Won), WIN);
        assert_eq!(outcome_color(&RoundOutcome::Push), PUSH);
        assert_eq!(outcome_color(&RoundOutcome::Lost), LOSE);
        assert_eq!(outcome_color(&RoundOutcome::Bust), LOSE);
    }
}
