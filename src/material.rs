use std::f64;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::{self, Ray},
    utility::random_float,
    vec3::{Vec3, dot, random_unit_vector, reflect, refract, unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scattered: &mut Ray,
        attenuation: &mut Color,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scattered: &mut Ray,
        attenuation: &mut Color,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: &Color) -> Metal {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scattered: &mut Ray,
        attenuation: &mut Color,
    ) -> bool {
        let reflected = reflect(ray_in.direction(), &hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo.clone();
        true
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric {
            refraction_index: ri,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi((1.0 - cosine), 5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scattered: &mut Ray,
        attenuation: &mut Color,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let mut ri = 0.0;
        if hit_record.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }

        let unit_direction = unit_vector(ray_in.direction());

        let cos_theta = f64::min(dot(&(-unit_direction), &hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_reflect = ri * sin_theta > 1.0;
        let mut direction = Vec3::default();

        if cannot_reflect || Self::reflectance(cos_theta, ri) > random_float() {
            direction = reflect(&unit_direction, &hit_record.normal)
        } else {
            direction = refract(&unit_direction, &hit_record.normal, ri)
        }

        *scattered = Ray::new(hit_record.p, direction);

        true
    }
}
