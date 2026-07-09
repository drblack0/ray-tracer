use std::{f64::consts::PI, rc::Rc};

use crate::{
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    utility::{random_float, random_float_by_range},
    vec3::{Point3, Vec3},
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

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(3.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(&albedo));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_from_range(0.5, 1.0);
                    let fuzz = random_float_by_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(&albedo));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5)));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = camera::Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.render(&world);
}
