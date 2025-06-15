mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let material_ground = material::Lambertian::new([0.8, 0.8, 0.0]);
    let material_center = material::Lambertian::new([0.1, 0.2, 0.5]);
    let material_left = material::Dielectric::new(1.5);
    let material_bubble = material::Dielectric::new(1.00 / 1.50);
    let material_right = material::Metal::new([0.8, 0.6, 0.2]);

    // world
    let mut world = hittable::HittableList::new();
    world.push(sphere::Sphere::new(
        [0.0, -100.5, -1.0],
        100.0,
        &material_ground,
    ));
    world.push(sphere::Sphere::new([0.0, 0.0, -1.2], 0.5, &material_center));
    world.push(sphere::Sphere::new([-1.0, 0.0, -1.0], 0.5, &material_left));
    world.push(sphere::Sphere::new(
        [-1.0, 0.0, -1.0],
        0.4,
        &material_bubble,
    ));
    world.push(sphere::Sphere::new([1.0, 0.0, -1.0], 0.5, &material_right));

    let cam = camera::Camera::new(
        aspect_ratio,
        image_width,
        100,
        50,
        20.0,
        [-2.0, 2.0, 1.0],
        [0.0, 0.0, -1.0],
        [0.0, 1.0, 0.0],
        10.0,
        3.4,
    );
    cam.render(&world);
}
