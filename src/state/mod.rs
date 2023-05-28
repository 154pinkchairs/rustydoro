use {
    serde::{Deserialize, Serialize},
    simple_home_dir::*,
    std::fs::{create_dir_all, File},
    std::io::{self, Write},
    std::path::PathBuf,
    std::result,
};

#[derive(Serialize, Deserialize)]
pub struct AppState {
    active_screen: Screen,
    current_idx: u16,   // index of the current timer (e.g. 2nd work session)
    current_cycle: u16, // a cycle is all work sessions and breaks between long breaks
    time_left: u16,
    work_duration: u16,
    break_duration: u16,
    long_break_duration: u16,
    long_break_interval: u16,
    paused: bool,
    projects: Vec<Project>,
    current_project: Project,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    description: String,
    estimated_time: u16,
    actual_time: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Screen {
    name: String,
    content: String,
    show_nav: bool,
    show_ctrl: bool,
}

const APPDATA_DIRS: [&str; 3] = [
    ".local/share/rustydoro",
    "Library/Application Support/rustydoro",
    "AppData/Local/rustydoro",
];

fn setup_dir() -> (result::Result<(), io::Error>, PathBuf) {
    let dir = match () {
        _ if cfg!(target_os = "windows") => APPDATA_DIRS[2],
        _ if cfg!(target_os = "macos") => APPDATA_DIRS[1],
        _ => APPDATA_DIRS[0],
    };

    let home = match home_dir() {
        Some(path) => path,
        None => panic!("Unable to find home directory"),
    };

    let dir = home.join(dir);

    if !dir.exists() {
        create_dir_all(&dir).unwrap_or_else(|e| {
            panic!("Unable to create directory {}: {}", dir.display(), e);
        });
    }

    (Ok(()), dir)
}

pub fn save_state(state: AppState) -> io::Result<()> {
    let (res, dir) = setup_dir();
    if res.is_err() {
        return res;
    }

    let mut file = File::create(dir.join("state.json"))?;
    let content = serde_json::to_string(&state)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn read_state() -> io::Result<AppState> {
    let (res, dir) = setup_dir();
    if res.is_err() {
        return Err(res.err().unwrap());
    }

    let file = File::open(dir.join("state.json"))?;
    let state: AppState = serde_json::from_reader(file)?;

    Ok(state)
}

// reset_timer sets the break/work timer's time left to the configured
// value for AppState.current_idx
pub fn reset_timer(state: &mut AppState) {
    let idx = state.current_idx as usize;
    let durations = [
        state.work_duration,
        state.break_duration,
        state.long_break_duration,
    ];
    state.time_left = durations[idx];
}

// onto_next handles moving between breaks and work sessions
pub fn onto_next(state: &mut AppState) {
    let durations = [
        state.work_duration,
        state.break_duration,
        state.long_break_duration,
    ];
    let sessions_size = durations.len() as u16;
    let mut next_idx = state.current_idx + 1;
    let mut next_cycle = state.current_cycle;
    if next_idx >= sessions_size {
        next_idx = 0;
        next_cycle += 1;
    }
    match state.active_screen.name.as_str() {
        "work" => {
            state.current_idx = next_idx;
            if state.current_idx / 2 == state.long_break_interval {
                state.active_screen.name = "long_break".to_string();
            } else {
                state.active_screen.name = "break".to_string();
            }
        }
        "break" => {
            state.current_idx = next_idx;
            state.active_screen.name = "work".to_string();
        }
        "long_break" => {
            if next_idx == 0 {
                state.current_cycle += 1;
            }
        }
        _ => {
            state.current_idx = 0;
            state.active_screen.name = "work".to_string();
        }
    }

    state.time_left = durations[state.current_idx as usize];
}
