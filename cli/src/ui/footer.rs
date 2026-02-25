use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::state::UiState;
use crate::ui::theme::{TOKIO_NIGHT_CYAN, TOKIO_NIGHT_MUTED, TOKIO_NIGHT_SUBTLE};

pub fn render_footer(frame: &mut Frame, area: Rect, ui: &UiState) {
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::styled(
        " >> ",
        Style::default()
            .fg(TOKIO_NIGHT_CYAN)
            .add_modifier(Modifier::BOLD),
    ));

    for (i, hint) in ui.footer.hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" │ ", Style::default().fg(TOKIO_NIGHT_SUBTLE)));
        }

        spans.push(Span::styled(
            format!("[{}]", hint.key),
            Style::default().fg(TOKIO_NIGHT_CYAN),
        ));
        spans.push(Span::styled(
            hint.label,
            Style::default().fg(TOKIO_NIGHT_MUTED),
        ));
    }

    let footer = Paragraph::new(Line::from(spans));
    frame.render_widget(footer, area);
}
