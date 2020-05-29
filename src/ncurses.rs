#![feature(exclusive_range_pattern)]

use super::ncurses_sys as sys;
use super::point::Point;

pub fn init() {
    unsafe {
        sys::initscr();
        sys::clear();
        sys::raw();
        // sys::start_color();
        // sys::cbreak();
        sys::keypad(sys::stdscr, true);
        sys::noecho();
    }
}

pub fn get_size() -> Point<usize> {
    let mut y = 0;
    let mut x = 0;
    unsafe {
        x = sys::getmaxx(sys::stdscr);
        y = sys::getmaxy(sys::stdscr);
    }

    Point {
        y: y as _,
        x: x as _,
    }
}

pub fn refresh() {
    unsafe {
        sys::refresh();
    }
}

pub fn mvprint(y: i32, x: i32, str: String) {
    use std::ffi::CString;
    let cstr = CString::new(str).unwrap();
    unsafe {
        sys::mvaddstr(y, x, cstr.as_ptr());
    }
}

pub fn quit() {
    unsafe {
        sys::endwin();
    }
}
