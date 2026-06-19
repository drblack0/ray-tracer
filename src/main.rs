use crate::color::{Color, write_color};
use std::f64;
use vec3::Vec3;

mod color;
mod vec3;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    let mut v = vec3::Vec3::new(4.0, 4.0, 4.0);
    let v2 = vec3::Vec3::new(2.0, 2.0, 2.0);
    v += v2;
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                f64::from(i) / f64::from(IMAGE_WIDTH - 1),
                f64::from(j) / f64::from(IMAGE_HEIGHT - 1),
                0.0,
            );
            write_color(color);
        }
    }
    eprintln!("\rDone.           ");
}
