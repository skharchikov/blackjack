use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::UiState;
use crate::ui::theme;

pub fn render_header(frame: &mut Frame, area: Rect, ui: &UiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::MUTED));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(28)])
        .split(inner);

    let title_line = Line::from(vec![
        Span::styled(
            ui.header.title.clone(),
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" — ", Style::default().fg(theme::MUTED)),
        Span::styled(ui.header.subtitle.clone(), Style::default().fg(theme::INFO)),
    ]);
    frame.render_widget(Paragraph::new(title_line), chunks[0]);

    let right_line = build_right_line(ui);
    frame.render_widget(
        Paragraph::new(right_line).alignment(Alignment::Right),
        chunks[1],
    );
}

fn build_right_line(ui: &UiState) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    if let Some(balance) = ui.header.my_balance {
        spans.push(Span::styled(
            format!("◉ {balance}"),
            Style::default().fg(theme::WIN),
        ));
    }

    Line::from(spans)
}
