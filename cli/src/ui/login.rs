use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{LoginState, LoginStatus};

pub fn render_login(frame: &mut Frame, area: Rect, login: &LoginState) {
    let block = Block::default().title("Login").borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Min(0),
    ])
    .split(inner);

    // Title
    let title = Paragraph::new(Line::from(vec![Span::styled(
        "Welcome to Blackjack",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Username input
    let username_block = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let cursor = if login.username.is_empty() { "_" } else { "" };
    let username_text = Paragraph::new(format!("{}{}", login.username, cursor))
        .block(username_block)
        .style(Style::default().fg(Color::White));
    frame.render_widget(username_text, chunks[1]);

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
    frame.render_widget(status, chunks[2]);
}
