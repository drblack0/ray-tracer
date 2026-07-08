use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(color: Color) {
    let x = (255.999 * color.x()) as i32;
    let y = (255.999 * color.y()) as i32;
    let z = (255.999 * color.z()) as i32;

    let r = color.x();
    let g = color.y();
    let b = color.z();

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    println!("{rbyte} {gbyte} {bbyte}")
}
