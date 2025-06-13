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
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b: f32 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            // write pixel to image file
            ppm_file
                .write_fmt(format_args!("{ir} {ig} {ib}\n"))
                .expect("Failed to write pixel");
        }
    }
    println!("\rDone                  ");
}
