use std::f64::consts::PI;

use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

pub fn random_min_max(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}
