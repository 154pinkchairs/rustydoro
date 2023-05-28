use crate::state;
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

pub fn handle_control_input(state: &mut state::AppState) {
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Char('w') => state::save_state(state).unwrap_or_else(|e| {
                eprintln!("Unable to save state: {}", e);
            }),
            Key::Char('r') => state::reset_timer(state),
        }
    }
}
