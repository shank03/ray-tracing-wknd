pub type Vec3 = [f64; 3];
pub type Point3 = Vec3;

pub fn init() -> Vec3 {
    [0.0, 0.0, 0.0]
}

pub trait SliceStruct {
    fn x(&self) -> &f64;
    fn y(&self) -> &f64;
    fn z(&self) -> &f64;
    fn display(&self) -> String;
}

impl SliceStruct for Vec3 {
    fn x(&self) -> &f64 {
        &self[0]
    }

    fn y(&self) -> &f64 {
        &self[1]
    }

    fn z(&self) -> &f64 {
        &self[2]
    }

    fn display(&self) -> String {
        format!("{} {} {}", self[0], self[1], self[2])
    }
}

pub trait SliceOp {
    fn len_squared(&self) -> f64;
    fn length(&self) -> f64;
    fn dot(self, rhs: Vec3) -> f64;
    fn cross(self, rhs: Vec3) -> Vec3;
    fn unit_vec(self) -> Vec3;

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
