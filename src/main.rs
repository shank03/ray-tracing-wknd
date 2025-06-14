mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // world
    let mut world = hittable::HittableList::new();
    world.push(sphere::Sphere::new([0.0, 0.0, -1.0], 0.5));
    world.push(sphere::Sphere::new([0.0, -100.5, -1.0], 100.0));

    let cam = camera::Camera::new(aspect_ratio, image_width, 100, 50);
    cam.render(&world);
}
