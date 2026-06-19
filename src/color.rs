use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) {
    let x = (255.999 * color.x()) as i32;
    let y = (255.999 * color.y()) as i32;
    let z = (255.999 * color.z()) as i32;

    println!("{x} {y} {z}")
}
