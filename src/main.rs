use std::rc::Rc;

use crate::{
    color::Color,
    hittable::HittableList,
    material::{Lambertian, Metal},
    vec3::Point3,
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

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.8)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    let mut camera = camera::Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
