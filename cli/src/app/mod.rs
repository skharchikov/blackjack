pub mod event;
pub mod keys;
pub mod state;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self as ct_event, Event};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::ui::render;
use event::AppEvent;
use keys::handle_key;
use state::App;

pub async fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    let (tx, mut rx) = mpsc::unbounded_channel::<AppEvent>();

    // Spawn keyboard/resize input reader on a blocking thread
    let tx_input = tx.clone();
    tokio::task::spawn_blocking(move || loop {
        if ct_event::poll(Duration::from_millis(250)).unwrap_or(false) {
            if let Ok(event) = ct_event::read() {
                let app_event = match event {
                    Event::Key(key_event) => {
                        if key_event.kind == ct_event::KeyEventKind::Press {
                            Some(AppEvent::Key(key_event.code))
                        } else {
                            None
                        }
                    }
                    Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
                    _ => None,
                };
                if let Some(evt) = app_event {
                    if tx_input.send(evt).is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Spawn tick timer
    let tx_tick = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            if tx_tick.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    // Main event loop
    loop {
        terminal.draw(|f| render(f, &app.ui))?;

        if let Some(event) = rx.recv().await {
            match event {
                AppEvent::Key(key) => {
                    handle_key(&mut app, key);
                }
                AppEvent::Tick => {
                    // Future: drive animations, poll server, etc.
                }
                AppEvent::Resize(_w, _h) => {
                    // ratatui handles resize automatically on next draw
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
