use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{
    card::{CardWidget, CARD_HEIGHT, CARD_WIDTH},
    history::render_history,
    layout::split_table_view,
    observers::render_observers,
    theme::TOKIO_NIGHT_BLUE,
    waiting_list::render_waiting_list,
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
    let Screen::Table(_) = ui.screen else { return };

    let block = Block::default()
        .title(" Board ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKIO_NIGHT_BLUE));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let has_betting = ui.betting.is_some();
    let constraints: Vec<Constraint> = if has_betting {
        vec![
            Constraint::Length(3),              // betting bar
            Constraint::Length(CARD_HEIGHT + 2), // dealer
            Constraint::Min(0),                 // players
        ]
    } else {
        vec![
            Constraint::Length(CARD_HEIGHT + 2), // dealer
            Constraint::Min(0),                 // players
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
    let Some(ref betting) = ui.betting else { return };
    let text = if betting.confirmed {
        format!(
            "Bet confirmed: {} chips  (waiting for round to start)",
            betting.current_bet
        )
    } else {
        format!(
            "Bet: {}  [← -{step}  → +{step}]   range: {}–{}",
            betting.current_bet,
            betting.min_bet,
            betting.max_bet,
            step = betting.step,
        )
    };
    let widget = Paragraph::new(text)
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
    let Screen::Table(ref table) = ui.screen else { return };

    let val_str = table
        .dealer
        .value
        .as_deref()
        .map(|v| format!(" = {v}"))
        .unwrap_or_default();
    let title = format!(" Dealer{val_str} ");

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    render_hand_cards(frame, inner, &table.dealer.cards, false);
}

fn render_players(frame: &mut Frame, area: Rect, ui: &UiState) {
    let Screen::Table(ref table) = ui.screen else { return };

    if table.players.is_empty() {
        let widget = Paragraph::new("  (no players at table)")
            .block(
                Block::default()
                    .title(" Players ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(widget, area);
        return;
    }

    let row_h = CARD_HEIGHT + 2; // cards + name line + border
    let constraints: Vec<Constraint> = table
        .players
        .iter()
        .map(|_| Constraint::Length(row_h))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, player) in table.players.iter().enumerate() {
        if i >= chunks.len() {
            break;
        }
        render_player_row(frame, chunks[i], player, table.phase);
    }
}

fn render_player_row(
    frame: &mut Frame,
    area: Rect,
    player: &crate::state::table::PlayerUiState,
    phase: GamePhase,
) {
    let border_color = if player.active {
        Color::Yellow
    } else if player.is_bust {
        Color::Red
    } else {
        Color::DarkGray
    };

    let is_my_turn = player.active && matches!(phase, GamePhase::PlayerTurn);
    let arrow = if player.active { "▶ " } else { "  " };
    let bet_part = player
        .bet
        .map(|b| format!("  bet:{b}"))
        .unwrap_or_default();
    let val_part = if player.hand_value > 0 {
        if player.is_bust {
            "  BUST".to_string()
        } else {
            format!("  ={}", player.hand_value)
        }
    } else {
        String::new()
    };

    let title_style = if is_my_turn {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let title = Line::from(vec![Span::styled(
        format!(" {}{}{}{} ", arrow, player.name, bet_part, val_part),
        title_style,
    )]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if player.hand.cards.is_empty() {
        let waiting = if player.bet.is_some() {
            "waiting for deal"
        } else {
            "place your bet"
        };
        let widget = Paragraph::new(waiting).style(Style::default().fg(Color::DarkGray));
        frame.render_widget(widget, inner);
    } else {
        render_hand_cards(frame, inner, &player.hand.cards, player.is_bust);
    }
}

fn render_hand_cards(
    frame: &mut Frame,
    area: Rect,
    cards: &[crate::state::cards::UiCard],
    busted: bool,
) {
    for (i, card) in cards.iter().enumerate() {
        let x = area.x + (i as u16) * (CARD_WIDTH + 1);
        if x + CARD_WIDTH > area.x + area.width {
            break;
        }
        let card_area = Rect::new(x, area.y, CARD_WIDTH, CARD_HEIGHT.min(area.height));

        match card.0 {
            Some(c) => {
                let style = if busted {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };
                CardWidget::new(&c).style(style).render(card_area, frame.buffer_mut());
            }
            None => render_hidden_card(frame, card_area),
        }
    }
}

fn render_hidden_card(frame: &mut Frame, area: Rect) {
    let lines = [
        "╭─────────╮",
        "│?????????│",
        "│?????????│",
        "│?????????│",
        "│?????????│",
        "│?????????│",
        "│?????????│",
        "│?????????│",
        "╰─────────╯",
    ];
    for (i, line) in lines.iter().enumerate() {
        if area.y + i as u16 >= area.y + area.height {
            break;
        }
        let row = Rect::new(area.x, area.y + i as u16, CARD_WIDTH, 1);
        let span = ratatui::text::Span::styled(*line, Style::default().fg(Color::DarkGray));
        span.render(row, frame.buffer_mut());
    }
}

// Required to call span.render on a Rect
use ratatui::widgets::Widget;
