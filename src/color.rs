use std::{fs, io::Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(file: &mut fs::File, pixel: Color) {
    let [r, g, b] = pixel;

    let range = 0.000..0.999;
    let [r, g, b] = [
        (256.0 * r.clamp(range.start, range.end)) as u8,
        (256.0 * g.clamp(range.start, range.end)) as u8,
        (256.0 * b.clamp(range.start, range.end)) as u8,
    ];

    file.write_fmt(format_args!("{r} {g} {b}\n"))
        .expect("Failed to write pixel");
}
