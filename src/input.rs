use std::sync::{Arc, Mutex};

use super::ncurses_sys as sys;

#[derive(Debug, PartialEq)]
pub enum Key {
    Escape(u32),

    Left,
    Right,
    Up,
    Down,

    Backspace,
    Enter,
    Home,
    End,
    Delete,
    Insert,
    PageUp,
    PageDown,

    Char(char),

    Unknown(u32),
}

#[derive(Debug)]
pub struct Input {
    pub key: Key,
    pub shift: bool,
    pub ctrl: bool,
}

pub fn getinput() -> Option<Input> {
    let ch = unsafe { sys::getch() } as u32;

    if ch == 0 {
        return None;
    }

    let mut key = Key::Unknown(ch);
    let mut shift = false;
    let mut ctrl = false;

    use std::convert::TryFrom;
    if let Ok(ch2) = char::try_from(ch) {
        if ch2.is_ascii() {
            key = Key::Char(ch2);
            shift = ch2.is_ascii_uppercase();
        }

        match ch2 {
            '\n' => key = Key::Enter,
            _ => {}
        }
    }

    match ch {
        27 => {
            let ch2 = unsafe { sys::getch() } as u32;
            key = Key::Escape(ch2)
        }
        sys::KEY_LEFT => key = Key::Left,
        sys::KEY_RIGHT => key = Key::Right,
        sys::KEY_UP => key = Key::Up,
        sys::KEY_DOWN => key = Key::Down,

        sys::KEY_BACKSPACE => key = Key::Backspace,
        sys::KEY_ENTER => key = Key::Enter,

        sys::KEY_HOME => key = Key::Home,
        sys::KEY_END => key = Key::End,
        330 => key = Key::Delete,
        331 => key = Key::Insert,
        339 => key = Key::PageUp,
        338 => key = Key::PageDown,
        _ => {}
    };

    Some(Input { key, shift, ctrl })
}

pub struct InputThread {
    state_handle: Arc<Mutex<Vec<Input>>>,
}

impl InputThread {
    pub fn new() -> Self {
        Self {
            state_handle: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn spawn(&self) {
        let my_handle = self.state_handle.clone();
        std::thread::spawn(move || loop {
            if let Some(ch) = getinput() {
                let mut inp = my_handle.lock().unwrap();
                inp.push(ch);
            }
        });
    }

    pub fn get(&self) -> Option<Input> {
        let my_handle = self.state_handle.clone();
        let mut state = my_handle.lock().unwrap();

        if state.is_empty() {
            return None;
        }

        Some(state.remove(0))
    }
}
