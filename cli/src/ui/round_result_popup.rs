use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{
    table::{RoundOutcome, RoundResult},
    UiState,
};

const COLOR_BG: Color = Color::Rgb(26, 27, 38);
const COLOR_COMMENT: Color = Color::Rgb(86, 95, 137);
const COLOR_GREEN: Color = Color::Rgb(158, 206, 106);
const COLOR_RED: Color = Color::Rgb(247, 118, 142);
const COLOR_YELLOW: Color = Color::Rgb(224, 175, 104);
const COLOR_CYAN: Color = Color::Rgb(125, 207, 255);

pub fn render_round_result_popup(frame: &mut Frame, area: Rect, ui: &UiState) {
    let crate::state::Screen::Table(ref table) = ui.screen else {
        return;
    };
    let Some(ref result) = table.round_result else {
        return;
    };

    let popup_area = centered_popup(44, 8, area);
    frame.render_widget(Clear, popup_area);

    let (border_color, outcome_color) = outcome_colors(&result.outcome);

    let block = Block::default()
        .title(Line::from(vec![Span::styled(
            " ROUND RESULT ",
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(COLOR_BG));

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
            Style::default().fg(COLOR_COMMENT),
        )]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}

fn build_payout_line(result: &RoundResult) -> Line<'static> {
    let (net_label, net_color) = match result.outcome {
        RoundOutcome::Lost | RoundOutcome::Bust => {
            (format!("bet {} → lost", result.bet), COLOR_RED)
        }
        RoundOutcome::Push => (format!("bet {} → returned", result.bet), COLOR_YELLOW),
        RoundOutcome::Won => (
            format!("bet {} → won +{}", result.bet, result.payout - result.bet),
            COLOR_GREEN,
        ),
        RoundOutcome::Blackjack => (
            format!(
                "bet {} → won +{}  🃏",
                result.bet,
                result.payout - result.bet
            ),
            COLOR_CYAN,
        ),
    };
    Line::from(vec![Span::styled(
        net_label,
        Style::default().fg(net_color),
    )])
}

fn outcome_colors(outcome: &RoundOutcome) -> (Color, Color) {
    match outcome {
        RoundOutcome::Blackjack => (COLOR_CYAN, COLOR_CYAN),
        RoundOutcome::Won => (COLOR_GREEN, COLOR_GREEN),
        RoundOutcome::Push => (COLOR_YELLOW, COLOR_YELLOW),
        RoundOutcome::Lost | RoundOutcome::Bust => (COLOR_RED, COLOR_RED),
    }
}

fn centered_popup(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
