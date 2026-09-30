use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{
    table::{RoundOutcome, RoundResult},
    UiState,
};
use crate::ui::theme;
use crate::ui::widgets::centered_rect;

pub fn render_round_result_popup(frame: &mut Frame, area: Rect, ui: &UiState) {
    let crate::state::Screen::Table(ref table) = ui.screen else {
        return;
    };
    let Some(ref result) = table.round_result else {
        return;
    };

    let popup_area = centered_rect(area, 44, 8);
    frame.render_widget(Clear, popup_area);

    let border_color = theme::outcome_color(&result.outcome);
    let outcome_color = border_color;

    let block = Block::default()
        .title(Line::from(vec![Span::styled(
            " ROUND RESULT ",
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::BG_POPUP));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // outcome
            Constraint::Length(1), // payout
            Constraint::Length(1), // spacer
            Constraint::Length(1), // dismiss hint
        ])
        .split(inner);

    // Outcome label
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            result.outcome.to_string(),
            Style::default()
                .fg(outcome_color)
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center),
        chunks[1],
    );

    // Payout line
    let payout_line = build_payout_line(result);
    frame.render_widget(
        Paragraph::new(payout_line).alignment(Alignment::Center),
        chunks[2],
    );

    // Dismiss hint
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            "press any key to dismiss",
            Style::default().fg(theme::MUTED),
        )]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}

fn build_payout_line(result: &RoundResult) -> Line<'static> {
    let net = result.payout.saturating_sub(result.bet);
    let label = match result.outcome {
        RoundOutcome::Lost | RoundOutcome::Bust => format!("bet {} → lost", result.bet),
        RoundOutcome::Push => format!("bet {} → returned", result.bet),
        RoundOutcome::Won => format!("bet {} → won +{net}", result.bet),
        RoundOutcome::Blackjack => format!("bet {} → won +{net}  🃏", result.bet),
    };
    Line::from(vec![Span::styled(
        label,
        Style::default().fg(theme::outcome_color(&result.outcome)),
    )])
}
