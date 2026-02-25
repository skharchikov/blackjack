use crossterm::event::KeyCode;

/// All events that flow through the application's event channel.
///
/// Future variants (e.g. `ServerMessage`, `ConnectionLost`) can be added
/// here without touching input or rendering layers.
#[derive(Debug)]
pub enum AppEvent {
    /// A key was pressed.
    Key(KeyCode),
    /// Periodic tick for animations / polling.
    Tick,
    /// Terminal was resized.
    Resize(u16, u16),
}
