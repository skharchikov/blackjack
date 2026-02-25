use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{LoginState, LoginStatus};

const BANNER: &[&str] = &[
    " ██████  ██       █████   ██████ ██   ██      ██  █████   ██████ ██   ██ ",
    " ██   ██ ██      ██   ██ ██      ██  ██       ██ ██   ██ ██      ██  ██  ",
    " ██████  ██      ███████ ██      █████        ██ ███████ ██      █████   ",
    " ██   ██ ██      ██   ██ ██      ██  ██  ██   ██ ██   ██ ██      ██  ██  ",
    " ██████  ███████ ██   ██  ██████ ██   ██  █████  ██   ██  ██████ ██   ██ ",
];

fn banner_color(line_index: usize, total: usize) -> Color {
    let t = line_index as f32 / (total - 1).max(1) as f32;
    let r = (0.0 + t * 120.0) as u8;
    let g = (255.0 - t * 60.0) as u8;
    let b = (255.0 - t * 80.0) as u8;
    Color::Rgb(r, g, b)
}

pub fn render_login(frame: &mut Frame, area: Rect, login: &LoginState) {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(BANNER.len() as u16 + 2), // banner + padding
        Constraint::Min(0),                          // flexible space
        Constraint::Length(3),                       // username input
        Constraint::Length(2),                       // status
    ])
    .split(inner);

    // Banner
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::default()); // top padding
    for (i, text) in BANNER.iter().enumerate() {
        let color = banner_color(i, BANNER.len());
        lines.push(Line::from(Span::styled(
            *text,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )));
    }

    let banner = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(banner, chunks[0]);

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
