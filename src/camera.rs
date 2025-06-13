use std::f64::INFINITY;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{SliceOp, SliceStruct},
};

pub struct Camera {}

impl Camera {
    fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
        let mut record = HitRecord::init();
        if world.hit(&r, 0.0..INFINITY, &mut record) {
            return record.normal.add([1.0, 1.0, 1.0]).mul_f(0.5);
        }

        let unit_direction = r.direction().unit_vec();
        let a = 0.5 * (unit_direction.y() + 1.0);
        [1.0, 1.0, 1.0].mul_f(1.0 - a).add([0.5, 0.7, 1.0].mul_f(a))
    }
}
