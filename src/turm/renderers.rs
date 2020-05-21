use super::canvas::Canvas;
use super::texel::Texel;

pub fn render_text(buf: &mut Canvas, r: usize, mut c: usize, text: String) {
    use std::cmp;

    let chars = text.chars();

    for ch in chars {
        if c >= buf.cols() {
            break;
        }
        buf.update(r, c, |mut t| {
            t.ch = ch;
            return t;
        });
        c += 1
    }
}

pub fn render_box<F: Fn(Texel, usize, usize) -> Texel>(
    buf: &mut Canvas,
    r: usize,
    c: usize,
    h: usize,
    w: usize,
    value_fn: F,
) {
    for ir in r..r + h {
        for ic in c..c + w {
            buf.update(ir, ic, |mut t| value_fn(t, ir, ic));
        }
    }
}

pub fn render_border<F: Fn(Texel, usize, usize) -> Texel>(
    buf: &mut Canvas,
    r: usize,
    c: usize,
    h: usize,
    w: usize,
    value_fn: F,
) {
    for ir in r..r + h {
        let ic = c;
        buf.update(ir, ic, |mut t| value_fn(t, ir, ic));
        let ic = c + w - 1;
        buf.update(ir, ic, |mut t| value_fn(t, ir, ic));
    }
    for ic in c..c + w {
        let ir = r;
        buf.update(ir, ic, |mut t| value_fn(t, ir, ic));
        let ir = r + h - 1;
        buf.update(ir, ic, |mut t| value_fn(t, ir, ic));
    }
}
