use std::{f64::INFINITY, fs, io::Write};

use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    util,
    vec3::{self, Point3, SliceOp, SliceStruct, Vec3},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_sample_scale: f64,
    sample_per_pixel: i32,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        sample_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        // calculate image height, it should be at least 1
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = image_height.max(1);

        let pixel_sample_scale = 1.0 / sample_per_pixel as f64;

        // camera
        let center = look_from;
        let theta = util::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // calculate the u,v,w unit basic vectors for the camera coordinate frame.
        let w = look_from.sub(look_at).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u.mul_f(viewport_width);
        let viewport_v = v.neg().mul_f(viewport_height);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u.div_f(image_width as f64);
        let pixel_delta_v = viewport_v.div_f(image_height as f64);

        // calculate the location of the upper left pixel
        let viewport_upper_left = center
            .sub(w.mul_f(focus_dist))
            .sub(viewport_u.div_f(2.0))
            .sub(viewport_v.div_f(2.0));
        let pixel00_loc = viewport_upper_left.add(pixel_delta_u.add(pixel_delta_v).mul_f(0.5));

        // calculate the camera defocus disk basis vectors
        let defocus_radis = focus_dist * util::degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u.mul_f(defocus_radis);
        let defocus_disk_v = v.mul_f(defocus_radis);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
            sample_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        // image file
        let mut ppm_file =
            fs::File::create("image.ppm").expect("Failed to create/truncate ppm image file");

        // write ppm header to image file
        ppm_file
            .write_fmt(format_args!(
                "P3\n{} {}\n255\n",
                self.image_width, self.image_height
            ))
            .expect("Failed to write header");

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            std::io::stdout().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = vec3::init();
                for _sample in 0..self.sample_per_pixel {
                    let r = self.get_ray(i, j);
                    let color = Self::ray_color(r, self.max_depth, world);
                    pixel_color.add_assign(color);
                }

                // write pixel to image file
                color::write_color(&mut ppm_file, pixel_color.mul_f(self.pixel_sample_scale));
            }
        }
        println!("\rDone                  ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = [util::random_float() - 0.5, util::random_float() - 0.5, 0.0];
        let pixel_sample = self
            .pixel00_loc
            .add(self.pixel_delta_u.mul_f(i as f64 + offset.x()))
            .add(self.pixel_delta_v.mul_f(j as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            let [x, y, _] = vec3::random_in_unit_disk();
            self.center
                .add(self.defocus_disk_u.mul_f(x))
                .add(self.defocus_disk_v.mul_f(y))
        };
        let ray_direction = pixel_sample.sub(ray_origin);
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return vec3::init();
        }

        let mut record = HitRecord::init();
        if let Some(mat) = world.hit(&r, 0.001..INFINITY, &mut record) {
            let mut scattered = Ray::init();
            let mut attenuation = vec3::init();
            if mat.scatter(&r, &record, &mut attenuation, &mut scattered) {
                return attenuation.mul(Self::ray_color(scattered, depth - 1, world));
            }
            return vec3::init();
        }

        let unit_direction = r.direction().unit_vec();
        let a = 0.5 * (unit_direction.y() + 1.0);
        [1.0, 1.0, 1.0].mul_f(1.0 - a).add([0.5, 0.7, 1.0].mul_f(a))
    }
}
