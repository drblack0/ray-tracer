use std::rc::Rc;

use crate::{
    color::Color,
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.list {
            if object.hit(
                r,
                &Interval {
                    min: interval.min,
                    max: closest_so_far,
                },
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }
        return hit_anything;
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = dot(ray.direction(), &outward_normal) < 0.0;

        if front_face {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal
        }
    }

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0))),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool;
}
