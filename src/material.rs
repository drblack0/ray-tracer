use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> bool;
}
