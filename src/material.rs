use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{self, SliceOp},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal.add(vec3::random_unit_vector());
        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().reflect(record.normal);
        let reflected = reflected
            .unit_vec()
            .add(vec3::random_unit_vector().mul_f(self.fuzz));

        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(record.normal) > 0.0
    }
}
