use ratatui::{layout::Rect, Frame};

use super::{
    history::render_history, layout::split_table_view, observers::render_observers,
    waiting_list::render_waiting_list,
};
use crate::state::UiState;

pub fn render_table_view(frame: &mut Frame, area: Rect, _ui: &UiState) {
    let layout = split_table_view(area);

    render_observers(frame, layout.observers);
    render_waiting_list(frame, layout.waiting_list);
    render_board(frame, layout.board);
    render_history(frame, layout.history);
}

fn render_board(frame: &mut Frame, area: Rect) {
    use ratatui::{
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
    };

    // Tokyo Night blue: #7aa2f7
    let border_color = Color::Rgb(122, 162, 247);

    let widget = Paragraph::new("")
        .block(
            Block::default()
                .title(" Board ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(widget, area);
}
