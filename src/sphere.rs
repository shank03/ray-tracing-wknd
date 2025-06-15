use crate::{
    hittable::Hittable,
    material::Material,
    vec3::{Point3, SliceOp},
};

pub struct Sphere<'m> {
    center: Point3,
    radius: f64,
    material: &'m dyn Material,
}
unsafe impl Send for Sphere<'_> {}
unsafe impl Sync for Sphere<'_> {}

impl<'m> Sphere<'m> {
    pub fn new(center: Point3, radius: f64, material: &'m dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'m> Hittable<'m> for Sphere<'m> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: std::ops::Range<f64>,
        record: &mut crate::hittable::HitRecord,
    ) -> Option<&'m dyn Material> {
        let oc = self.center.sub(*r.origin());
        let a = r.direction().len_squared();
        let h = r.direction().dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= ray_t.start || ray_t.end <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.start || ray_t.end <= root {
                return None;
            }
        }

        record.t = root;
        record.p = r.at(root);

        let outward_normal = record.p.sub(self.center).div_f(self.radius);
        record.set_face_normal(r, outward_normal);

        Some(self.material)
    }
}
