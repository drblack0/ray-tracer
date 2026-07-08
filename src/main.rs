use crate::{hittable::HittableList, vec3::Point3};

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

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
