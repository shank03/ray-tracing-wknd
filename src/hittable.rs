use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::{self, Point3, SliceOp, Vec3},
};

#[derive(Clone)]
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

#[repr(transparent)]
pub struct HittableList<H>(Vec<H>);

impl<H: Hittable> HittableList<H> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: H) {
        self.0.push(item);
    }
}

impl<H: Hittable> IntoIterator for HittableList<H> {
    type Item = H;

    type IntoIter = std::vec::IntoIter<H>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::init();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.end;

        for obj in self.0.iter() {
            if obj.hit(r, ray_t.start..closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec.clone();
            }
        }

        hit_anything
    }
}
