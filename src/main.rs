use vec3::SliceOp;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    // collect random materials
    let mut materials = Vec::new();
    for a in -9..10 {
        for b in -9..10 {
            let random_mat = util::random_float();
            let center = [
                a as f64 + 0.9 * util::random_float(),
                0.2,
                b as f64 + 0.9 * util::random_float(),
            ];

            let diff = center.sub([4.0, 0.2, 0.0]);
            if diff.length() > 0.9 {
                let material = match random_mat {
                    x if x < 0.8 => {
                        let albedo = vec3::random().mul(vec3::random());
                        material::MatType::Lambertian(material::Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedo = vec3::random_min_max(0.5, 1.0);
                        let fuzz = util::random_min_max(0.0, 0.5);
                        material::MatType::Metal(material::Metal::new(albedo, fuzz))
                    }
                    _ => material::MatType::Dielectric(material::Dielectric::new(1.5)),
                };

                materials.push((center, material));
            }
        }
    }

    // world
    let mut world = hittable::HittableList::new();

    let material_ground = material::Lambertian::new([0.5, 0.5, 0.5]);
    world.push(sphere::Sphere::new(
        [0.0, -1000.0, 0.0],
        1000.0,
        &material_ground,
    ));

    materials.iter().for_each(|(center, mat)| {
        world.push(sphere::Sphere::new(
            *center,
            0.2,
            match mat {
                material::MatType::Lambertian(lambertian) => lambertian,
                material::MatType::Metal(metal) => metal,
                material::MatType::Dielectric(dielectric) => dielectric,
            },
        ));
    });

    let mat_dielectric = material::Dielectric::new(1.5);
    let mat_lambertian = material::Lambertian::new([0.4, 0.2, 0.1]);
    let mat_metal = material::Metal::new([0.7, 0.6, 0.5], 0.0);

    world.push(sphere::Sphere::new([0.0, 1.0, 0.0], 1.0, &mat_dielectric));
    world.push(sphere::Sphere::new([-4.0, 1.0, 0.0], 1.0, &mat_lambertian));
    world.push(sphere::Sphere::new([4.0, 1.0, 0.0], 1.0, &mat_metal));

    let cam = camera::Camera::new(
        16.0 / 9.0,
        1920,
        64,
        16,
        24.0,
        [13.0, 2.0, 5.0],
        vec3::init(),
        [0.0, 1.0, 0.0],
        0.6,
        10.0,
    );
    cam.render(&world);
}
