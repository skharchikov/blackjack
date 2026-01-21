use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::state::{LoginState, LoginStatus};

pub fn render_login(frame: &mut Frame, area: Rect, login: &LoginState) {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(8), // BigText with PixelSize::Full needs 8 lines
        Constraint::Min(0),    // Flexible space
        Constraint::Length(3), // Username input
        Constraint::Length(2), // Status
    ])
    .split(inner);

    // Title
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .lines(vec!["BLACKJACK".into()])
        .alignment(Alignment::Center)
        .build();

    frame.render_widget(big_text, chunks[0]);

    // Username input
    let username_block = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let cursor = if login.username.is_empty() { "_" } else { "" };
    let username_text = Paragraph::new(format!("{}{}", login.username, cursor))
        .block(username_block)
        .style(Style::default().fg(Color::White));
    frame.render_widget(username_text, chunks[2]);

    // Status
    let (status_text, status_color) = match &login.status {
        LoginStatus::Idle => ("Enter your username to continue", Color::DarkGray),
        LoginStatus::Connecting => ("Connecting...", Color::Yellow),
        LoginStatus::Error(msg) => (msg.as_str(), Color::Red),
    };

    let status = Paragraph::new(Line::from(Span::styled(
        status_text,
        Style::default().fg(status_color),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(status, chunks[3]);
}
