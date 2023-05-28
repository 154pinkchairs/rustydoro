mod keys;
mod pomui;
mod timers;
pub mod state;

use {
    crate::keys::{input, NAVKEYS, NAV_HINTS},
    std::io::{stdin, stdout, Write},
    std::sync::mpsc,
    std::thread,
    std::time::Duration,
    termion::event::Key,
    termion::input::TermRead,
    std::fs;
    std::path::Path;
    termion::raw::IntoRawMode,
    tui::backend::TermionBackend,
    tui::Terminal,
};

fn main() {
    // First, show the available break intervals in a list
    let mut stdout = stdout().into_raw_mode().unwrap_or_else(|e| {
        panic!("Unable to set raw mode: {}", e);
    });
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap_or_else(|e| {
        panic!("Unable to start terminal: {}", e);
    });
    // use only the whitelisted keys listed in NAVKEYS
    let input_keys: String = NAVKEYS.iter().map(|key| format!("{:?}", key)).collect();

    let key_hints: Vec<String> = NAV_HINTS.iter().map(|hint| hint.to_string()).collect();

    terminal.clear().unwrap_or_else(|e| {
        panic!("Unable to clear terminal: {}", e);
    });

    terminal
        .draw(|mut f| {
            let chunks = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints(
                    [
                        tui::layout::Constraint::Percentage(10),
                        tui::layout::Constraint::Percentage(80),
                        tui::layout::Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = tui::widgets::Block::default()
                .title("Break Timer")
                .borders(tui::widgets::Borders::ALL);
            f.render_widget(block, chunks[0]);

            let block = tui::widgets::Block::default().borders(tui::widgets::Borders::ALL);
            f.render_widget(block, chunks[1]);

            let block = tui::widgets::Block::default().borders(tui::widgets::Borders::ALL);
            f.render_widget(block, chunks[2]);

            let text = tui::widgets::Paragraph::new(input_keys.as_ref());
            f.render_widget(text, chunks[0]);

            let text = tui::widgets::Paragraph::new(key_hints.join("\n").as_ref());
            f.render_widget(text, chunks[2]);
        })
        .unwrap_or_else(|e| {
            panic!("Unable to draw terminal: {}", e);
        });

    let stdin = stdin();
}
