use super::average::Average;
use super::grid::Grid;
use super::texel::Texel;

pub struct Canvas {
    buf: Grid<Texel>,
    frames: usize,

    orig_termios: libc::termios,
    last_wait: std::time::SystemTime,
    last_elapsed: std::time::Duration,
    elapsed_target: std::time::Duration,
    elapsed_average: Average,
}

impl Canvas {
    pub fn new(rows: usize, cols: usize) -> Self {
        use super::libc_util::*;
        use std::time::{Duration, SystemTime};

        let buf = Grid::new(rows, cols, Texel::default());
        Self {
            buf,
            frames: 0,
            orig_termios: new_termios(),
            last_wait: SystemTime::now(),
            last_elapsed: Duration::default(),
            elapsed_target: Duration::from_secs_f64(1.0 / 60.0),
            elapsed_average: Average::new(7),
        }
    }

    // GETTERS

    pub fn rows(&self) -> usize {
        return self.buf.rows;
    }

    pub fn cols(&self) -> usize {
        return self.buf.cols;
    }

    pub fn frames(&self) -> usize {
        return self.frames;
    }

    pub fn fps(&self) -> f64 {
        return 1.0 / self.elapsed_average.avg.as_secs_f64();
    }

    pub fn update<F: Fn(Texel) -> Texel>(&mut self, r: usize, c: usize, updater: F) {
        if r >= self.buf.rows || c >= self.buf.cols {
            return;
        }

        let old_texel = &self.buf.data[r][c];
        let new_texel = updater(self.buf.data[r][c].clone());

        if new_texel.z_index >= old_texel.z_index {
            self.buf.data[r][c] = new_texel;
        }
    }

    pub fn update_each<F: Fn(Texel, usize, usize) -> Texel>(&mut self, updater: F) {
        for r in 0..self.buf.rows {
            for c in 0..self.buf.cols {
                self.buf.data[r][c] = updater(self.buf.data[r][c].clone(), r, c);
            }
        }
    }

    pub fn init(&mut self) {
        use super::ansi::*;
        self.orig_termios = enable_raw_mode();
    }

    pub fn cleanup(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.orig_termios);
        }
    }

    pub fn update_size(&mut self) {
        use super::ansi::*;
        if let Some(rc) = get_window_size() {
            if rc.row != self.buf.rows || rc.col != self.buf.cols {
                self.buf.resize(rc.row, rc.col, Texel::default());
                self.update_each(|mut t, r, c| {
                    t.repaint = true;
                    return t;
                })
            }
        }
    }

    pub fn paint(&mut self) {
        use super::ansi;
        use std::io::Write;

        let mut f = std::io::stdout();

        let mut last_bg_color = ansi::Color::Normal;
        let mut last_fg_color = ansi::Color::Normal;

        ansi::save_cursor(&mut f);
        ansi::hide_cursor(&mut f);
        for r in 0..self.buf.rows {
            for c in 0..self.buf.cols {
                let cell = &mut self.buf.data[r][c];

                if cell.should_repaint() {
                    ansi::set_cursor(&mut f, r, c);
                    if cell.fg != last_fg_color {
                        ansi::set_fg(&mut f, &cell.fg);
                        last_fg_color = cell.fg.clone();
                    }
                    if cell.bg != last_bg_color {
                        ansi::set_bg(&mut f, &cell.bg);
                        last_bg_color = cell.bg.clone();
                    }
                    write!(&mut f, "{}", cell.ch);
                    cell.update_prev();
                }
            }
        }
        ansi::load_cursor(&mut f);
        ansi::show_cursor(&mut f);
        self.frames += 1;
        f.flush();
    }

    pub fn wait(&mut self) {
        // Delay to match target.
        let elapsed = self.last_wait.elapsed().unwrap();
        if self.elapsed_target > elapsed {
            let sleep_for = self.elapsed_target - elapsed;
            std::thread::sleep(sleep_for);
        }

        // Calculate fps
        self.last_elapsed = self.last_wait.elapsed().unwrap();
        self.elapsed_average.push(self.last_elapsed.clone());

        // Update wait time
        self.last_wait = std::time::SystemTime::now();
    }

    pub fn no_wait(&mut self) {
        // Calculate fps
        self.last_elapsed = self.last_wait.elapsed().unwrap();
        self.elapsed_average.push(self.last_elapsed.clone());

        // Update wait time
        self.last_wait = std::time::SystemTime::now();
    }
}
