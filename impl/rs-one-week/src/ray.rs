use crate::vec3::{self, Point3, SliceOp, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn init() -> Self {
        Self {
            origin: vec3::init(),
            direction: vec3::init(),
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.add(self.direction.mul_f(t))
    }
}
