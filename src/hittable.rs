use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::{self, Point3, SliceOp, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn init() -> Self {
        Self {
            p: vec3::init(),
            normal: vec3::init(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            // ray is outside the sphere
            outward_normal
        } else {
            // ray is inside the sphere
            outward_normal.neg()
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, record: &mut HitRecord) -> bool;
}
