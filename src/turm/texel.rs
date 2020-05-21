use super::ansi;

#[derive(Clone)]
pub struct Texel {
    pub ch: char,
    pub bg: ansi::Color,
    pub fg: ansi::Color,

    pub z_index: usize,
    pub repaint: bool,

    // Previous state
    prev_ch: char,
    prev_bg: ansi::Color,
    prev_fg: ansi::Color,
}

impl Texel {
    pub fn should_repaint(&self) -> bool {
        if self.repaint {
            return true;
        }

        if self.ch != self.prev_ch {
            return true;
        }
        if self.bg != self.prev_bg {
            return true;
        }
        if self.fg != self.prev_fg {
            return true;
        }

        return false;
    }

    pub fn clear(&mut self) {
        self.ch = ' ';
        self.bg = ansi::Color::Normal;
        self.fg = ansi::Color::Normal;
    }

    pub fn update_prev(&mut self) {
        self.prev_ch = self.ch;
        self.prev_bg = self.bg.clone();
        self.prev_fg = self.fg.clone();

        self.clear();

        self.repaint = false;
    }
}

impl Default for Texel {
    fn default() -> Self {
        Self {
            ch: ' ',
            bg: ansi::Color::Normal,
            fg: ansi::Color::Normal,

            // Previous state
            prev_ch: ' ',
            prev_bg: ansi::Color::Normal,
            prev_fg: ansi::Color::Normal,

            z_index: 0,
            repaint: true,
        }
    }
}
