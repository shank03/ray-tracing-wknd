use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::{self, Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn init() -> Self {
        Self {
            p: vec3::init(),
            normal: vec3::init(),
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, record: &mut HitRecord) -> bool;
}
