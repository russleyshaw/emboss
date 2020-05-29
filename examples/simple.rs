extern crate crss;

use crss::*;

use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};

fn main() {
    use std::ffi::CStr;

    let mut input = InputThread::new();
    let mut timer = TimerBuilder::new().size(7).fps(60.0).build();
    let mut quit = false;
    let mut frame = 0;

    crss::init();
    input.spawn();
    while !quit {
        if let Some(input) = input.get() {
            mvprint(0, 0, format!("{:?}", input));

            if Key::Char('q') == input.key {
                quit = true;
            }
        }

        mvprint(1, 0, format!("{:?}", timer.fps()));
        progress_bar(2, 0, 100, timer.frame() as f64 / 1000.0);

        crss::refresh();
        timer.trigger();
    }

    crss::quit();
}
