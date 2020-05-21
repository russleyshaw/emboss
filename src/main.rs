mod turm;

use turm::*;

fn read_char() -> u8 {
    let mut buffer = [0; 1];
    let buffer_ptr: *mut libc::c_void = &mut buffer as *mut _ as *mut libc::c_void;

    unsafe {
        libc::read(libc::STDIN_FILENO, buffer_ptr, 1);
    }

    return buffer[0];
}

use std::sync::{Arc, Mutex};
fn spawn_stdin_reader() -> Arc<Mutex<Vec<u8>>> {
    let input_lock = Arc::new(Mutex::new(Vec::<u8>::new()));

    let thread_lock = input_lock.clone();
    std::thread::spawn(move || loop {
        let ch = read_char();
        if ch != 0 {
            let mut vec = thread_lock.lock().unwrap();
            vec.push(ch);
        }
    });

    return input_lock;
}

fn main() {
    let mut tbuf = Canvas::new(20, 100);
    let input_lock = spawn_stdin_reader();

    tbuf.init();
    tbuf.update_size();
    let mut last_input = String::new();
    let mut total_input = String::new();
    loop {
        let frames = tbuf.frames();

        tbuf.update_size();

        let reader_lock = input_lock.clone();
        let input = reader_lock.lock().unwrap().pop();
        if let Some(ch) = input {
            let ch = ch as char;
            if ch == 'q' {
                break;
            }

            last_input = format!("Last Input: {:?}", ch);
            total_input = format!("{}{}", total_input, ch);
        }

        let fps_text = format!("AVG FPS {:.3}", tbuf.fps());
        render_box(&mut tbuf, 2, 0, 5, fps_text.len() + 2, |mut t, r, c| {
            t.ch = ' ';
            t.bg = Color::Red;
            t.fg = Color::Yellow;
            return t;
        });
        render_border(&mut tbuf, 2, 0, 5, fps_text.len() + 2, |mut t, r, c| {
            t.ch = '#';
            return t;
        });
        render_text(&mut tbuf, 3, 1, last_input.clone());
        render_text(&mut tbuf, 4, 1, fps_text);

        // Show Corners

        let updater = |mut t: Texel| {
            t.ch = 'X';
            return t;
        };
        tbuf.update(0, 0, updater);
        tbuf.update(0, tbuf.cols() - 1, updater);
        tbuf.update(tbuf.rows() - 1, tbuf.cols() - 1, updater);
        tbuf.update(tbuf.rows() - 1, 0, updater);
        render_text(&mut tbuf, 1, 1, format!("{}", frames));
        render_text(&mut tbuf, 1, 4, total_input.clone());

        tbuf.paint();
        tbuf.no_wait();
    }
    tbuf.cleanup();
}
