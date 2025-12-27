use std::time::Duration;

use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    let block = Block::default()
        .title("Blackjack DeFi")
        .borders(Borders::ALL);

    let greeting = Paragraph::new("Blackjack DeFi (press 'q' to quit)")
        .block(block)
        .centered();

    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        let q_pressed = event::read()
            .context("event read failed")?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'));
        return Ok(q_pressed);
    }
    Ok(false)
}
