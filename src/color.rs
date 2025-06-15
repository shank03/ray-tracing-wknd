use std::{fs, io::Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}

pub fn write_color(file: &mut fs::File, pixel: Color) {
    let [r, g, b] = pixel;
    let [r, g, b] = [linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b)];

    let range = 0.000..0.999;
    let [r, g, b] = [
        (256.0 * r.clamp(range.start, range.end)) as u8,
        (256.0 * g.clamp(range.start, range.end)) as u8,
        (256.0 * b.clamp(range.start, range.end)) as u8,
    ];

    file.write_fmt(format_args!("{r} {g} {b}\n"))
        .expect("Failed to write pixel");
}
