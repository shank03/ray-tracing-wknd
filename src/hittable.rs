use std::ops::Range;

use crate::{
    material::Material,
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

pub trait Hittable<'m>: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, record: &mut HitRecord) -> Option<&'m dyn Material>;
}

#[repr(transparent)]
pub struct HittableList<H>(Vec<H>);

impl<'m, H: Hittable<'m>> HittableList<H> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: H) {
        self.0.push(item);
    }
}

impl<'m, H: Hittable<'m>> IntoIterator for HittableList<H> {
    type Item = H;

    type IntoIter = std::vec::IntoIter<H>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'m, H: Hittable<'m>> Hittable<'m> for HittableList<H> {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, record: &mut HitRecord) -> Option<&'m dyn Material> {
        let mut temp_rec = HitRecord::init();
        let mut closest_so_far = ray_t.end;
        let mut material_hit = None;

        for obj in self.0.iter() {
            if let Some(m) = obj.hit(r, ray_t.start..closest_so_far, &mut temp_rec) {
                material_hit = Some(m);
                closest_so_far = temp_rec.t;
                *record = temp_rec.clone();
            }
        }

        material_hit
    }
}
