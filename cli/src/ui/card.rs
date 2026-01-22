use std::iter::zip;

use blackjack_core::domain::{Card, Rank, Suit};
use indoc::indoc;
use ratatui::prelude::*;

/// Card widget dimensions: 8 characters wide × 5 lines tall
pub const CARD_WIDTH: u16 = 8;
pub const CARD_HEIGHT: u16 = 5;

/// A wrapper around the domain Card for rendering in the terminal.
#[derive(Debug, Clone, Copy)]
pub struct CardWidget<'a> {
    pub card: &'a Card,
    pub style: Style,
}

impl<'a> CardWidget<'a> {
    pub fn new(card: &'a Card) -> Self {
        Self {
            card,
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

fn rank_template(rank: Rank) -> &'static str {
    match rank {
        Rank::Ace => indoc! {"
            ╭──────╮
            │Ax  x │
            │      │
            │ x  xA│
            ╰──────╯"},
        Rank::Two => indoc! {"
            ╭──────╮
            │2x  x │
            │      │
            │ x  x2│
            ╰──────╯"},
        Rank::Three => indoc! {"
            ╭──────╮
            │3x  x │
            │      │
            │ x  x3│
            ╰──────╯"},
        Rank::Four => indoc! {"
            ╭──────╮
            │4x  x │
            │      │
            │ x  x4│
            ╰──────╯"},
        Rank::Five => indoc! {"
            ╭──────╮
            │5x  x │
            │      │
            │ x  x5│
            ╰──────╯"},
        Rank::Six => indoc! {"
            ╭──────╮
            │6x  x │
            │      │
            │ x  x6│
            ╰──────╯"},
        Rank::Seven => indoc! {"
            ╭──────╮
            │7x  x │
            │      │
            │ x  x7│
            ╰──────╯"},
        Rank::Eight => indoc! {"
            ╭──────╮
            │8x  x │
            │      │
            │ x  x8│
            ╰──────╯"},
        Rank::Nine => indoc! {"
            ╭──────╮
            │9x  x │
            │      │
            │ x  x9│
            ╰──────╯"},
        Rank::Ten => indoc! {"
            ╭──────╮
            │10  x │
            │      │
            │ x  10│
            ╰──────╯"},
        Rank::Jack => indoc! {"
            ╭──────╮
            │Jx    │
            │  JJ  │
            │    xJ│
            ╰──────╯"},
        Rank::Queen => indoc! {"
            ╭──────╮
            │Qx    │
            │  QQ  │
            │    xQ│
            ╰──────╯"},
        Rank::King => indoc! {"
            ╭──────╮
            │Kx    │
            │  KK  │
            │    xK│
            ╰──────╯"},
    }
}

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Clubs => Color::Black,
        Suit::Diamonds => Color::Red,
        Suit::Hearts => Color::Red,
        Suit::Spades => Color::Black,
    }
}

fn suit_symbol(suit: Suit) -> char {
    match suit {
        Suit::Clubs => '♣',
        Suit::Diamonds => '♦',
        Suit::Hearts => '♥',
        Suit::Spades => '♠',
    }
}

impl Widget for CardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let template = rank_template(self.card.rank);
        let symbol = suit_symbol(self.card.suit);
        let card = template.replace('x', &symbol.to_string());

        let fg = self.style.fg.unwrap_or(suit_color(self.card.suit));
        let bg = self.style.bg.unwrap_or(Color::Reset);

        for (line, row) in zip(card.lines(), area.rows()) {
            let span = Span::raw(line).fg(fg).bg(bg);
            span.render(row, buf);
        }
    }
}
