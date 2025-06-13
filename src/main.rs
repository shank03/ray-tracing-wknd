use std::{f64::INFINITY, fs, io::Write};

use vec3::{SliceOp, SliceStruct};

mod color;
mod hittable;
mod ray;
mod sphere;
mod util;
mod vec3;

fn ray_color(r: ray::Ray, world: &dyn hittable::Hittable) -> color::Color {
    let mut record = hittable::HitRecord::init();
    if world.hit(&r, 0.0..INFINITY, &mut record) {
        return record.normal.add([1.0, 1.0, 1.0]).mul_f(0.5);
    }

    let unit_direction = r.direction().unit_vec();
    let a = 0.5 * (unit_direction.y() + 1.0);
    [1.0, 1.0, 1.0].mul_f(1.0 - a).add([0.5, 0.7, 1.0].mul_f(a))
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // calculate image height, it should be at least 1
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = image_height.max(1);

    // world
    let mut world = hittable::HittableList::new();
    world.push(sphere::Sphere::new([0.0, 0.0, -1.0], 0.5));
    world.push(sphere::Sphere::new([0.0, -100.5, -1.0], 100.0));

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = vec3::init();

    // calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = [viewport_width, 0.0, 0.0];
    let viewport_v = [0.0, -viewport_height, 0.0];

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u.div_f(image_width as f64);
    let pixel_delta_v = viewport_v.div_f(image_height as f64);

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        .sub([0.0, 0.0, focal_length])
        .sub(viewport_u.div_f(2.0))
        .sub(viewport_v.div_f(2.0));
    let pixel00_loc = viewport_upper_left.add(pixel_delta_u.add(pixel_delta_v).mul_f(0.5));

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
            let pixel_center = pixel00_loc
                .add(pixel_delta_u.mul_f(i as f64))
                .add(pixel_delta_v.mul_f(j as f64));
            let ray_direction = pixel_center.sub(camera_center);
            let r = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, &world);

            // write pixel to image file
            color::write_color(&mut ppm_file, pixel_color);
        }
    }
    println!("\rDone                  ");
}
