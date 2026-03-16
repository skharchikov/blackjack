use blackjack_core::domain::{Card, DeckId, Rank, Suit};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::state::login::LoginField;
use crate::state::{LoginState, LoginStatus};
use crate::ui::card::{CardWidget, CARD_HEIGHT, CARD_WIDTH};
use crate::ui::theme::{TOKIO_NIGHT_CYAN, TOKIO_NIGHT_MUTED, TOKIO_NIGHT_SUBTLE};

const BANNER: &[&str] = &[
    " ██████  ██       █████   ██████ ██   ██      ██  █████   ██████ ██   ██ ",
    " ██   ██ ██      ██   ██ ██      ██  ██       ██ ██   ██ ██      ██  ██  ",
    " ██████  ██      ███████ ██      █████        ██ ███████ ██      █████   ",
    " ██   ██ ██      ██   ██ ██      ██  ██  ██   ██ ██   ██ ██      ██  ██  ",
    " ██████  ███████ ██   ██  ██████ ██   ██  █████  ██   ██  ██████ ██   ██ ",
];

// banner is 75 chars + border(2) + padding(4) = 81
const FORM_WIDTH: u16 = 81;
// top-pad(1) + banner(5) + space(2) + label+input(2) + space(1) + label+input(2) + space(1) + status(1) + border(2) = 17
const FORM_HEIGHT: u16 = 17;

const SCATTER_CARDS: &[(Rank, Suit)] = &[
    (Rank::Ace, Suit::Spades),
    (Rank::King, Suit::Hearts),
    (Rank::Seven, Suit::Diamonds),
    (Rank::Queen, Suit::Clubs),
    (Rank::Jack, Suit::Spades),
    (Rank::Ten, Suit::Hearts),
    (Rank::Three, Suit::Diamonds),
    (Rank::Eight, Suit::Clubs),
    (Rank::Five, Suit::Hearts),
    (Rank::Two, Suit::Spades),
    (Rank::Nine, Suit::Diamonds),
    (Rank::Four, Suit::Clubs),
    (Rank::Six, Suit::Hearts),
    (Rank::Ace, Suit::Diamonds),
    (Rank::King, Suit::Spades),
];

fn banner_color(line_index: usize, total: usize) -> Color {
    let t = line_index as f32 / (total - 1).max(1) as f32;
    let r = (0.0 + t * 120.0) as u8;
    let g = (255.0 - t * 60.0) as u8;
    let b = (255.0 - t * 80.0) as u8;
    Color::Rgb(r, g, b)
}

pub fn render_login(frame: &mut Frame, area: Rect, login: &LoginState) {
    render_scatter_bg(frame, area);

    let form_area = center(area, FORM_WIDTH, FORM_HEIGHT);
    render_login_form(frame, form_area, login);
}

fn render_login_form(frame: &mut Frame, area: Rect, login: &LoginState) {
    frame.render_widget(Clear, area);

    let form_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKIO_NIGHT_CYAN))
        .padding(Padding::horizontal(2));

    let inner = form_block.inner(area);
    frame.render_widget(form_block, area);

    let chunks = Layout::vertical([
        Constraint::Length(1),                   // top padding
        Constraint::Length(BANNER.len() as u16), // banner
        Constraint::Length(2),                   // spacing
        Constraint::Length(2),                   // username (label + input)
        Constraint::Length(1),                   // spacing
        Constraint::Length(2),                   // password (label + input)
        Constraint::Length(1),                   // spacing
        Constraint::Length(1),                   // status
    ])
    .split(inner);

    // Banner
    let lines: Vec<Line> = BANNER
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let color = banner_color(i, BANNER.len());
            Line::from(Span::styled(
                *text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
        })
        .collect();
    let banner = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(banner, chunks[1]);

    // Center the input fields (40 chars wide within the form)
    let field_width: u16 = 40;

    // Username field
    let username_area = center_horizontal(chunks[3], field_width);
    render_field(
        frame,
        username_area,
        "Username",
        &login.username,
        false,
        login.active_field == LoginField::Username,
    );

    // Password field
    let password_area = center_horizontal(chunks[5], field_width);
    render_field(
        frame,
        password_area,
        "Password",
        &login.password,
        true,
        login.active_field == LoginField::Password,
    );

    // Status
    let (status_text, status_color) = match &login.status {
        LoginStatus::Idle => ("Press Enter to continue", Color::DarkGray),
        LoginStatus::Connecting => ("Connecting...", Color::Yellow),
        LoginStatus::Error(msg) => (msg.as_str(), Color::Red),
    };

    let status = Paragraph::new(Line::from(Span::styled(
        status_text,
        Style::default().fg(status_color),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(status, chunks[7]);
}

/// Renders a single-line input field:
///   Label
///   value___________________________
fn render_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    masked: bool,
    active: bool,
) {
    if area.height < 2 {
        return;
    }

    let label_area = Rect { height: 1, ..area };
    let input_area = Rect {
        y: area.y + 1,
        height: 1,
        ..area
    };

    // Label
    let label_color = if active {
        TOKIO_NIGHT_CYAN
    } else {
        TOKIO_NIGHT_MUTED
    };
    let label_widget = Paragraph::new(Span::styled(label, Style::default().fg(label_color)));
    frame.render_widget(label_widget, label_area);

    // Input line: underlined text + underlined padding
    let display = if masked {
        "*".repeat(value.chars().count())
    } else {
        value.to_string()
    };
    let cursor = if active { "▎" } else { "" };
    let text = format!("{}{}", display, cursor);
    let fill_len =
        (input_area.width as usize).saturating_sub(UnicodeWidthStr::width(text.as_str()));
    let padding: String = " ".repeat(fill_len);

    let line_color = if active {
        TOKIO_NIGHT_CYAN
    } else {
        TOKIO_NIGHT_SUBTLE
    };

    let underlined = Modifier::UNDERLINED;
    let input_line = Line::from(vec![
        Span::styled(
            &text,
            Style::default().fg(Color::White).add_modifier(underlined),
        ),
        Span::styled(
            padding,
            Style::default().fg(line_color).add_modifier(underlined),
        ),
    ]);

    let input_widget = Paragraph::new(input_line);
    frame.render_widget(input_widget, input_area);
}

fn render_scatter_bg(frame: &mut Frame, area: Rect) {
    if area.height < CARD_HEIGHT || area.width < CARD_WIDTH {
        return;
    }

    let dimmed = Style::default().fg(TOKIO_NIGHT_SUBTLE);

    let cols = (area.width / (CARD_WIDTH + 2)) as usize;
    let rows = (area.height / (CARD_HEIGHT + 1)) as usize;

    if cols == 0 || rows == 0 {
        return;
    }

    let mut card_idx = 0;
    for row in 0..rows {
        for col in 0..cols {
            let x = area.x + col as u16 * (CARD_WIDTH + 2);
            let y = area.y + row as u16 * (CARD_HEIGHT + 1);

            if x + CARD_WIDTH > area.x + area.width || y + CARD_HEIGHT > area.y + area.height {
                continue;
            }

            let (rank, suit) = SCATTER_CARDS[card_idx % SCATTER_CARDS.len()];
            let card = Card::new(DeckId::One, suit, rank);
            let widget = CardWidget::new(&card).style(dimmed);

            let card_area = Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT);
            frame.render_widget(widget, card_area);

            card_idx += 1;
        }
    }
}

fn center(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);

    let [_, hcenter, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(w),
            Constraint::Min(0),
        ])
        .flex(Flex::Center)
        .areas(area);

    let [_, vcenter, _] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(h),
            Constraint::Min(0),
        ])
        .flex(Flex::Center)
        .areas(hcenter);

    vcenter
}

fn center_horizontal(area: Rect, width: u16) -> Rect {
    let w = width.min(area.width);

    let [_, centered, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(w),
            Constraint::Min(0),
        ])
        .flex(Flex::Center)
        .areas(area);

    centered
}
