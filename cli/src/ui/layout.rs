use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct ScreenLayout {
    pub header: Rect,
    pub main: Rect,
    pub footer: Rect,
}

pub fn split_screen(area: Rect) -> ScreenLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // main
            Constraint::Length(1), // footer
        ])
        .split(area);

    ScreenLayout {
        header: chunks[0],
        main: chunks[1],
        footer: chunks[2],
    }
}

/// Layout for the Blackjack Table View
///
/// ┌────────────────────────────────────────┬──────────┐
/// │              Board                     │ Observers│
/// │                                        ├──────────┤
/// ├────────────────────────────────────────┤ Waiting  │
/// │  History (10 lines)                    │          │
/// └────────────────────────────────────────┴──────────┘
pub struct TableLayout {
    pub observers: Rect,
    pub waiting_list: Rect,
    pub board: Rect,
    pub history: Rect,
}

pub fn split_table_view(area: Rect) -> TableLayout {
    // Split into left content (85%) and right sidebar (15%)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85), // left: board + history
            Constraint::Percentage(15), // right: observers + waiting list
        ])
        .split(area);

    // Left column: board (main) + history (fixed 10 lines at bottom)
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10),
        ])
        .split(columns[0]);

    // Right column: observers (top) + waiting list (bottom)
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(columns[1]);

    TableLayout {
        board: left[0],
        history: left[1],
        observers: right[0],
        waiting_list: right[1],
    }
}
