use std::{fs, io::Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(file: &mut fs::File, pixel: Color) {
    let [r, g, b] = pixel;

    let [r, g, b] = [
        (255.999 * r) as i32,
        (255.999 * g) as i32,
        (255.999 * b) as i32,
    ];

    file.write_fmt(format_args!("{r} {g} {b}\n"))
        .expect("Failed to write pixel");
}
