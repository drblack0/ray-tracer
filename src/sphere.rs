use crate::ray::Ray;
use crate::vec3::dot;
use crate::{
    hittable::{HitRecord, Hittable},
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(point: Point3, radius: f64) -> Sphere {
        Sphere {
            center: point,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let oc = &self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = h * h - a * c;
        if disc < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(disc);

        let mut root = h - sqrtd / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = h + sqrtd / a;

            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(&r, outward_normal);

        return true;
    }
}
