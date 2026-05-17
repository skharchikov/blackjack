use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{
    history::render_history, layout::split_table_view, observers::render_observers,
    theme::TOKIO_NIGHT_BLUE, waiting_list::render_waiting_list,
};
use crate::state::{table::GamePhase, Screen, UiState};

pub fn render_table(frame: &mut Frame, area: Rect, ui: &UiState) {
    let layout = split_table_view(area);

    render_observers(frame, layout.observers);
    render_waiting_list(frame, layout.waiting_list);
    render_board(frame, layout.board, ui);
    render_history(frame, layout.history);
}

fn render_board(frame: &mut Frame, area: Rect, ui: &UiState) {
    let Screen::Table(_) = ui.screen else {
        return;
    };

    let block = Block::default()
        .title(" Board ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKIO_NIGHT_BLUE));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split inner: betting bar (if active) | dealer area | players area
    let has_betting = ui.betting.is_some();
    let constraints = if has_betting {
        vec![
            Constraint::Length(3), // betting bar
            Constraint::Length(4), // dealer
            Constraint::Min(0),    // players
        ]
    } else {
        vec![
            Constraint::Length(4), // dealer
            Constraint::Min(0),    // players
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    let (dealer_chunk, players_chunk) = if has_betting {
        render_betting_bar(frame, chunks[0], ui);
        (chunks[1], chunks[2])
    } else {
        (chunks[0], chunks[1])
    };

    render_dealer(frame, dealer_chunk, ui);
    render_players(frame, players_chunk, ui);
}

fn render_betting_bar(frame: &mut Frame, area: Rect, ui: &UiState) {
    let Some(ref betting) = ui.betting else {
        return;
    };

    let status = if betting.confirmed {
        format!("Bet confirmed: {} chips  (waiting for round to start)", betting.current_bet)
    } else {
        format!(
            "Bet: {} chips   [← -{}  → +{}]   range: {}–{}",
            betting.current_bet, betting.step, betting.step, betting.min_bet, betting.max_bet
        )
    };

    let widget = Paragraph::new(status)
        .block(
            Block::default()
                .title(" Bet ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(widget, area);
}

fn render_dealer(frame: &mut Frame, area: Rect, ui: &UiState) {
    let Screen::Table(ref table) = ui.screen else {
        return;
    };

    let cards_str = if table.dealer.cards.is_empty() {
        "  (no cards yet)".to_string()
    } else {
        let hand: Vec<String> = table.dealer.cards.iter().map(|c| c.display()).collect();
        let val = table.dealer.value.as_deref().unwrap_or("?");
        format!("  {} = {}", hand.join("  "), val)
    };

    let widget = Paragraph::new(cards_str)
        .block(
            Block::default()
                .title(" Dealer ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(widget, area);
}

fn render_players(frame: &mut Frame, area: Rect, ui: &UiState) {
    let Screen::Table(ref table) = ui.screen else {
        return;
    };

    if table.players.is_empty() {
        let widget = Paragraph::new("  (no players at table)")
            .block(
                Block::default()
                    .title(" Players ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(TOKIO_NIGHT_BLUE)),
            )
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(widget, area);
        return;
    }

    // One row per player
    let n = table.players.len();
    let constraints: Vec<Constraint> =
        (0..n).map(|_| Constraint::Length(3)).collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, player) in table.players.iter().enumerate() {
        if i >= chunks.len() {
            break;
        }
        render_player_row(frame, chunks[i], player, ui);
    }
}

fn render_player_row(
    frame: &mut Frame,
    area: Rect,
    player: &crate::state::table::PlayerUiState,
    ui: &UiState,
) {
    let Screen::Table(ref table) = ui.screen else {
        return;
    };

    let border_color = if player.active {
        Color::Yellow
    } else if player.is_bust {
        Color::Red
    } else {
        Color::DarkGray
    };

    let active_marker = if player.active { "▶ " } else { "  " };
    let bet_str = match player.bet {
        Some(b) => format!("  bet:{}", b),
        None => String::new(),
    };
    let balance_str = format!("  bal:{}", player.balance);

    let cards_part = if player.hand.cards.is_empty() {
        "(no cards)".to_string()
    } else {
        let hand: Vec<String> = player.hand.cards.iter().map(|c| c.display()).collect();
        let val = if player.is_bust {
            "BUST".to_string()
        } else {
            player.hand_value.to_string()
        };
        format!("{} = {}", hand.join("  "), val)
    };

    let status_part = if !player.status.is_empty() && player.status != "waiting" && player.status != "playing" && player.status != "bet placed" {
        format!("  [{}]", player.status)
    } else {
        String::new()
    };

    // Highlight "it's your turn" differently based on phase
    let is_my_turn = player.active && matches!(table.phase, GamePhase::PlayerTurn);

    let title_style = if is_my_turn {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let title = Line::from(vec![
        Span::styled(
            format!("{}{}{}{}", active_marker, player.name, bet_str, balance_str),
            title_style,
        ),
    ]);

    let content = Line::from(vec![
        Span::styled(
            format!("  {}{}", cards_part, status_part),
            Style::default().fg(if player.is_bust {
                Color::Red
            } else {
                Color::White
            }),
        ),
    ]);

    let widget = Paragraph::new(content).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );
    frame.render_widget(widget, area);
}
