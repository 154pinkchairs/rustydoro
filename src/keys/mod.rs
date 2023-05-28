// key codes of arrow keys, enter, space, escape, backspace and h/j
use termion::event::Key;

pub mod input;

pub const NAVKEYS: [Key; 12] = [
    Key::Up,
    Key::PageUp,
    Key::Down,
    Key::PageDown,
    Key::Char('\n'),
    Key::Char(' '),
    Key::Esc,
    Key::Backspace,
    Key::Char('h'),
    Key::Char('j'),
    Key::Char('?'),
    Key::Char('q'),
];

pub const CTRL_KEYS: [Key; 4] = [
    Key::Char('c'),
    Key::Char('r'),
    Key::Char('s'),
    Key::Char('w'),
];

pub const CTRL_HINTS: [&str; 4] = ["c - continue", "r - reset", "s - skip", "w - save progress"];

pub const NAV_HINTS: [&str; 8] = [
    "↑, k - up",
    "↓, j - down",
    "PageUp/Down - last/first",
    "Enter - confirm",
    "Space - toggle",
    "Esc/q - quit",
    "Backspace - previous screen",
    "? - help",
];
