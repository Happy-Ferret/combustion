//! Line drawing routines
//!
//! All of these take a `plot` function to draw a single pixel on the image.

use super::plot::plot_gaussian_dot;

/// Uses Xiaolin Wu's algorithm to draw a line, but where it would normally draw a single pixel it instead draws a thick dot determined
/// by a Gaussian function, giving the appearance of a thick, anti-aliased line with controllable width and hardness (falloff).
pub fn draw_line_thick_gaussian<F>(x0: i64, y0: i64, x1: i64, y1: i64, width: f64, hardness: f64, mut plot: F) where F: FnMut(i64, i64, f64) {
    draw_line_xiaolin_wu(x0, y0, x1, y1, |x, y, alpha| {
        plot_gaussian_dot(x, y, alpha, width, hardness, &mut plot);
    });
}

/// Uses Bresenham's algorithm to draw an aliased line.
pub fn draw_line_bresenham<F>(mut x0: i64, mut y0: i64, x1: i64, y1: i64, mut plot: F) where F: FnMut(i64, i64, f64) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        plot(x0, y0, 1.0);

        if x0 == x1 && y0 == y1 { break; }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Uses Xiaolin Wu's algorithm to draw an anti-aliased line.
pub fn draw_line_xiaolin_wu<F>(x0: i64, y0: i64, x1: i64, y1: i64, mut plot: F) where F: FnMut(i64, i64, f64) {
    use std::mem::swap;

    let mut plot_float = |x: f64, y: f64, opacity: f64| {
        plot(x as i64, y as i64, opacity)
    };

    let mut x0 = x0 as f64;
    let mut y0 = y0 as f64;
    let mut x1 = x1 as f64;
    let mut y1 = y1 as f64;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let gradient = if dx < 0.0001 { 1.0 } else { dy / dx };

    let xend = x0.round();
    let yend = y0 + gradient * (xend - x0);

    let xgap = 1.0 - (x0 + 0.5).fract();

    let xpxl1 = xend;
    let ypxl1 = yend.trunc();

    if steep {
        plot_float(ypxl1, xpxl1, (1.0 - yend.fract()) * xgap);
        plot_float(ypxl1 + 1.0, xpxl1, yend.fract() * xgap);
    } else {
        plot_float(xpxl1, ypxl1, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl1, ypxl1 + 1.0, yend.fract() * xgap);
    }

    let mut intery = yend + gradient;

    let xend = x1.round();
    let yend = y1 + gradient * (xend - x1);
    let xgap = (x1 + 0.5).fract();

    let xpxl2 = xend;
    let ypxl2 = yend.trunc();

    if steep {
        plot_float(ypxl2, xpxl2, (1.0 - yend.fract()) * xgap);
        plot_float(ypxl2 + 1.0, xpxl2, yend.fract() * xgap);
    } else {
        plot_float(xpxl2, ypxl2, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl2, ypxl2 + 1.0, yend.fract() * xgap);
    }

    let mut x = xpxl1 + 1.0;

    if steep {
        while x <= (xpxl2 - 1.0) {
            let y = intery.trunc();

            plot_float(y, x, 1.0 - intery.fract());
            plot_float(y + 1.0, x, intery.fract());

            intery += gradient;
            x += 1.0;
        }
    } else {
        while x <= (xpxl2 - 1.0) {
            let y = intery.trunc();

            plot_float(x, y, 1.0 - intery.fract());
            plot_float(x, y + 1.0, intery.fract());

            intery += gradient;
            x += 1.0;
        }
    }
}