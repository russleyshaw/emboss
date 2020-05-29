use super::ncurses_sys as sys;
use std::ffi::CString;
use std::io::Write;

pub fn progress_bar(y: usize, x: usize, w: usize, progress: f64) {
    let progress = progress.min(1.0).max(0.0);
    let completed_w = ((w as f64) * progress).round() as usize;
    let txt = format!(" {}% ", (progress * 100.0) as usize);

    unsafe {
        sys::move_(y as i32, x as i32);

        for _ in 0..completed_w {
            sys::addch('#' as u32);
        }

        for _ in completed_w..w {
            sys::addch(' ' as u32);
        }

        let text_w = x + (w / 2) - txt.len() / 2;
        let txt = CString::new(txt).unwrap();
        sys::mvaddstr(y as i32, text_w as i32, txt.as_ptr());
    }
}
