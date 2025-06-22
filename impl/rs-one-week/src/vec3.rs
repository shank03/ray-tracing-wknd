use crate::util;

pub type Vec3 = [f64; 3];
pub type Point3 = Vec3;

pub fn init() -> Vec3 {
    [0.0, 0.0, 0.0]
}

pub fn random() -> Vec3 {
    [
        util::random_float(),
        util::random_float(),
        util::random_float(),
    ]
}

pub fn random_min_max(min: f64, max: f64) -> Vec3 {
    [
        util::random_min_max(min, max),
        util::random_min_max(min, max),
        util::random_min_max(min, max),
    ]
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_min_max(-1.0, 1.0);
        let len_sq = p.len_squared();
        // prevent fp leak; very small value can underflow to zero
        // use 1^-160 as min bound
        if 1e-160 < len_sq && len_sq <= 1.0 {
            return p.div_f(len_sq.sqrt());
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = [
            util::random_min_max(-1.0, 1.0),
            util::random_min_max(-1.0, 1.0),
            0.0,
        ];
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

pub trait SliceOp {
    fn len_squared(&self) -> f64;
    fn length(&self) -> f64;
    fn dot(self, rhs: Vec3) -> f64;
    fn cross(self, rhs: Vec3) -> Vec3;
    fn unit_vec(self) -> Vec3;
    fn near_zero(&self) -> bool;
    fn reflect(self, n: Vec3) -> Vec3;
    fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3;

    fn neg(&self) -> Vec3;
    fn add(self, rhs: Vec3) -> Vec3;
    fn add_assign(&mut self, rhs: Vec3);
    fn sub(self, rhs: Vec3) -> Vec3;
    fn mul(self, rhs: Vec3) -> Vec3;
    fn mul_f(self, rhs: f64) -> Vec3;
    fn div_f(self, rhs: f64) -> Vec3;
}

impl SliceOp for Vec3 {
    fn len_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn length(&self) -> f64 {
        self.len_squared().sqrt()
    }

    fn dot(self, rhs: Vec3) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    fn cross(self, rhs: Vec3) -> Vec3 {
        [
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]
    }

    fn unit_vec(self) -> Vec3 {
        let len = self.length();
        self.div_f(len)
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    fn reflect(self, n: Vec3) -> Vec3 {
        self.sub(n.mul_f(2.0 * self.dot(n)))
    }

    fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = self.neg().dot(n).min(1.0);
        let r_out_perp = self.add(n.mul_f(cos_theta)).mul_f(etai_over_etat);
        let r_out_parallel = n.mul_f(-(1.0 - r_out_perp.len_squared()).abs().sqrt());
        r_out_perp.add(r_out_parallel)
    }

    fn neg(&self) -> Vec3 {
        [-self[0], -self[1], -self[2]]
    }

    fn add(self, rhs: Vec3) -> Vec3 {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }

    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }

    fn sub(self, rhs: Vec3) -> Vec3 {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }

    fn mul(self, rhs: Vec3) -> Vec3 {
        [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    }

    fn mul_f(self, rhs: f64) -> Vec3 {
        [self[0] * rhs, self[1] * rhs, self[2] * rhs]
    }

    fn div_f(self, rhs: f64) -> Vec3 {
        self.mul_f(1.0 / rhs)
    }
}
