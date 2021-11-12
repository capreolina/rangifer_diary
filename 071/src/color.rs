use crate::util::{PI_2, PI_2_3};
use plotters::style::RGBColor;

pub fn sample_palette(x: f32) -> RGBColor {
    let x = x * PI_2;

    RGBColor(
        (255.0 * (x.sin() + 1.0) / 2.0) as u8,
        (255.0 * ((x + PI_2_3).sin() + 1.0) / 2.0) as u8,
        (255.0 * ((x + PI_2_3 + PI_2_3).sin() + 1.0) / 2.0) as u8,
    )
}

/// - <https://en.wikipedia.org/wiki/Rec._601>
/// - <https://en.wikipedia.org/wiki/Luma_(video)>
/// - <https://www.w3.org/TR/AERT/#color-contrast>
pub fn luma(color: RGBColor) -> f32 {
    let RGBColor(r, g, b) = color;

    0.299 * (r as f32 / 255.0)
        + 0.587 * (g as f32 / 255.0)
        + 0.114 * (b as f32 / 255.0)
}

pub fn is_dark(color: RGBColor) -> bool {
    luma(color) < 0.5
}

pub fn mix<I: Iterator<Item = RGBColor>>(iter: I) -> Option<RGBColor> {
    let mut n = 0u32;
    let (mut r_sq_sum, mut g_sq_sum, mut b_sq_sum) = (0, 0, 0);
    for c in iter {
        n += 1;

        let RGBColor(r8, g8, b8) = c;
        let (r, g, b) = (r8 as u64, g8 as u64, b8 as u64);
        let (r_sq, g_sq, b_sq) = (r * r, g * g, b * b);

        r_sq_sum += r_sq;
        g_sq_sum += g_sq;
        b_sq_sum += b_sq;
    }

    if n < 1 {
        return None;
    }
    let n = n as f64;

    Some(RGBColor(
        (r_sq_sum as f64 / n).sqrt().round().min(255.0).max(0.0) as u8,
        (g_sq_sum as f64 / n).sqrt().round().min(255.0).max(0.0) as u8,
        (b_sq_sum as f64 / n).sqrt().round().min(255.0).max(0.0) as u8,
    ))
}
