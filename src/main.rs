use std::{fs, io::Write};

mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    // image file
    let mut ppm_file =
        fs::File::create("image.ppm").expect("Failed to create/truncate ppm image file");

    // write ppm header to image file
    ppm_file
        .write_fmt(format_args!("P3\n{image_width} {image_height}\n255\n"))
        .expect("Failed to write header");

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        std::io::stdout().flush().unwrap();

        for i in 0..image_width {
            let pixel_color = [
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            ];

            // write pixel to image file
            color::write_color(&mut ppm_file, pixel_color);
        }
    }
    println!("\rDone                  ");
}
