use std::iter::zip;
use std::sync::LazyLock;

use bj_core::domain::{Card, Rank, Suit};
use indoc::indoc;
use ratatui::prelude::*;

use crate::ui::theme;

/// Card widget dimensions: 11 characters wide × 9 lines tall
pub const CARD_WIDTH: u16 = 11;
pub const CARD_HEIGHT: u16 = 9;

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

// Each template is exactly 11 chars wide × 9 lines tall.
// 'x' is replaced with the suit symbol at build time.
fn rank_template(rank: Rank) -> &'static str {
    match rank {
        Rank::Ace => indoc! {"
            ╭─────────╮
            │Ax      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      xA│
            ╰─────────╯"},
        Rank::Two => indoc! {"
            ╭─────────╮
            │2x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x2│
            ╰─────────╯"},
        Rank::Three => indoc! {"
            ╭─────────╮
            │3x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x3│
            ╰─────────╯"},
        Rank::Four => indoc! {"
            ╭─────────╮
            │4x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x4│
            ╰─────────╯"},
        Rank::Five => indoc! {"
            ╭─────────╮
            │5x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x5│
            ╰─────────╯"},
        Rank::Six => indoc! {"
            ╭─────────╮
            │6x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x6│
            ╰─────────╯"},
        Rank::Seven => indoc! {"
            ╭─────────╮
            │7x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x7│
            ╰─────────╯"},
        Rank::Eight => indoc! {"
            ╭─────────╮
            │8x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x8│
            ╰─────────╯"},
        Rank::Nine => indoc! {"
            ╭─────────╮
            │9x      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      x9│
            ╰─────────╯"},
        Rank::Ten => indoc! {"
            ╭─────────╮
            │10      x│
            │         │
            │         │
            │    x    │
            │         │
            │         │
            │x      10│
            ╰─────────╯"},
        Rank::Jack => indoc! {"
            ╭─────────╮
            │Jx       │
            │         │
            │  J   J  │
            │  JJJJJ  │
            │  J   J  │
            │         │
            │       xJ│
            ╰─────────╯"},
        Rank::Queen => indoc! {"
            ╭─────────╮
            │Qx       │
            │         │
            │  Q   Q  │
            │  QQQQQ  │
            │  Q   Q  │
            │         │
            │       xQ│
            ╰─────────╯"},
        Rank::King => indoc! {"
            ╭─────────╮
            │Kx       │
            │         │
            │  K  K   │
            │  KKK    │
            │  K  K   │
            │         │
            │       xK│
            ╰─────────╯"},
    }
}

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Clubs | Suit::Spades => theme::TEXT,
        Suit::Diamonds | Suit::Hearts => theme::CARD_RED,
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

// Rank::Two = 2 .. Rank::Ace = 14  →  index = rank as usize - 2  (13 ranks)
// Suit repr 0..3                    →  index = suit as usize       (4 suits)
static CARD_STRINGS: LazyLock<[[Box<str>; 4]; 13]> = LazyLock::new(|| {
    const RANKS: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs];
    RANKS.map(|rank| {
        SUITS.map(|suit| {
            let template = rank_template(rank);
            let symbol = suit_symbol(suit);
            let mut s = String::with_capacity(template.len());
            for ch in template.chars() {
                if ch == 'x' {
                    s.push(symbol);
                } else {
                    s.push(ch);
                }
            }
            s.into_boxed_str()
        })
    })
});

impl Widget for CardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let card_str = &CARD_STRINGS[self.card.rank as usize - 2][self.card.suit as usize];

        let fg = self.style.fg.unwrap_or(suit_color(self.card.suit));

        for (line, row) in zip(card_str.lines(), area.rows()) {
            let mut span = Span::raw(line).fg(fg);
            if let Some(bg) = self.style.bg {
                span = span.bg(bg);
            }
            span.render(row, buf);
        }
    }
}
