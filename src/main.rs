use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableList},
    ray::Ray,
    utility::INFINITY,
    vec3::{Point3, unit_vector},
};

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(
        ray,
        &interval::Interval {
            min: 0.0,
            max: INFINITY,
        },
        &mut rec,
    ) {
        return 0.5 * rec.normal + Color::new(1.0, 1.0, 1.0);
    }

    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut camera = camera::Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
