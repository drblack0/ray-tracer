use rand::Rng;

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_float_by_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * random_float();
}
