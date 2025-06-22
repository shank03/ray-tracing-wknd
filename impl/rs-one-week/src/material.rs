use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    util,
    vec3::{self, SliceOp},
};

pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

fn reflectance(cosine: f64, ri: f64) -> f64 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = record.normal.add(vec3::random_unit_vector());
                // catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = record.normal;
                }

                *scattered = Ray::new(record.p, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = r_in.direction().reflect(record.normal);
                let reflected = reflected
                    .unit_vec()
                    .add(vec3::random_unit_vector().mul_f(*fuzz));

                *scattered = Ray::new(record.p, reflected);
                *attenuation = *albedo;

                scattered.direction().dot(record.normal) > 0.0
            }
            Material::Dielectric(refraction_index) => {
                *attenuation = [1.0, 1.0, 1.0];
                let ri = if record.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction().unit_vec();
                let cos_theta = unit_direction.neg().dot(record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let direction =
                    if cannot_refract || reflectance(cos_theta, ri) > util::random_float() {
                        unit_direction.reflect(record.normal)
                    } else {
                        unit_direction.refract(record.normal, ri)
                    };

                *scattered = Ray::new(record.p, direction);
                true
            }
        }
    }
}
