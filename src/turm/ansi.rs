use super::rowcol::*;
use std::io::Write;

pub fn enable_raw_mode() -> libc::termios {
    use super::libc_util::*;

    let mut raw = new_termios();
    let mut orig_termios = raw.clone();
    unsafe {
        libc::tcgetattr(libc::STDIN_FILENO, &mut raw);
        orig_termios = raw.clone();

        // raw.c_iflag &= ~(libc::ICRNL | libc::IXON);
        libc::cfmakeraw(&mut raw);
        raw.c_oflag &= !(libc::OPOST);
        raw.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);
        raw.c_cflag |= libc::CS8;
        raw.c_cc[libc::VMIN] = 0;
        raw.c_cc[libc::VTIME] = 1;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw);
    };

    return orig_termios;
}

pub fn clear<W: Write>(f: &mut W) {
    write!(f, "\x1b[2J");
}

pub fn clear_row<W: Write>(f: &mut W, row: usize) {
    write!(f, "\x1b[K{}", row + 1);
}

pub fn set_cursor<W: Write>(f: &mut W, r: usize, c: usize) {
    write!(f, "\x1b[{};{}H", r + 1, c + 1);
}

pub fn set_normal<W: Write>(f: &mut W) {
    write!(f, "\x1b[0m");
}

#[derive(Clone, PartialEq)]
pub enum Color {
    Normal,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

pub fn set_bg<W: Write>(f: &mut W, color: &Color) {
    let c = match color {
        Color::Normal => 49,
        Color::Black => 40,
        Color::Red => 41,
        Color::Green => 42,
        Color::Yellow => 43,
        Color::Blue => 44,
        Color::Magenta => 45,
        Color::Cyan => 46,
        Color::White => 47,
        Color::BrightBlack => 100,
        Color::BrightRed => 101,
        Color::BrightGreen => 102,
        Color::BrightYellow => 103,
        Color::BrightBlue => 104,
        Color::BrightMagenta => 105,
        Color::BrightCyan => 106,
        Color::BrightWhite => 107,
    };

    write!(f, "\x1b[{}m", c);
}

pub fn set_fg<W: Write>(f: &mut W, color: &Color) {
    let c = match color {
        Color::Normal => 39,
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::BrightBlack => 90,
        Color::BrightRed => 91,
        Color::BrightGreen => 92,
        Color::BrightYellow => 93,
        Color::BrightBlue => 94,
        Color::BrightMagenta => 95,
        Color::BrightCyan => 96,
        Color::BrightWhite => 97,
    };

    write!(f, "\x1b[{}m", c);
}

pub fn hide_cursor<W: Write>(f: &mut W) {
    write!(f, "\x1b[?25l");
}
pub fn show_cursor<W: Write>(f: &mut W) {
    write!(f, "\x1b[?25h");
}

pub fn next_line<W: Write>(f: &mut W) {
    write!(f, "\x1b[E");
}

pub fn get_window_size() -> Option<RowCol> {
    use super::libc_util::*;

    let mut ws = new_winsize();
    let result = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws) };
    if result == -1 || ws.ws_col == 0 {
        return None;
    }

    Some(RowCol {
        col: ws.ws_col as usize,
        row: ws.ws_row as usize,
    })
}

pub fn save_cursor<W: Write>(f: &mut W) {
    write!(f, "\x1b[s");
}

pub fn load_cursor<W: Write>(f: &mut W) {
    write!(f, "\x1b[u");
}
