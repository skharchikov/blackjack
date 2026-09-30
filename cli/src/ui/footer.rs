use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::state::UiState;
use crate::ui::theme;

pub fn render_footer(frame: &mut Frame, area: Rect, ui: &UiState) {
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::styled(
        " >> ",
        Style::default()
            .fg(theme::INFO)
            .add_modifier(Modifier::BOLD),
    ));

    for (i, hint) in ui.footer.hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" │ ", Style::default().fg(theme::SUBTLE)));
        }

        spans.push(Span::styled(
            format!("[{}]", hint.key),
            Style::default().fg(theme::INFO),
        ));
        spans.push(Span::styled(hint.label, Style::default().fg(theme::MUTED)));
    }

    let footer = Paragraph::new(Line::from(spans));
    frame.render_widget(footer, area);
}
