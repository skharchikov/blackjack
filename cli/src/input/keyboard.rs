use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};

pub fn read_key() -> Result<Option<KeyCode>> {
    if event::poll(Duration::from_millis(250))? {
        if let Some(key) = event::read()?.as_key_press_event() {
            return Ok(Some(key.code));
        }
    }
    Ok(None)
}
