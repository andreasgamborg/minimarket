use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::db::Database;

pub fn update(app: &mut App, db: &Database, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Right => app.increment_counter(),
        KeyCode::Left => app.decrement_counter(),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            app.barcode_input.push(c);
        },
        KeyCode::Enter => app.process_barcode(db),
        _ => {}
    };
}
