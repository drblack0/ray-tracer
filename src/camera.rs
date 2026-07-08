use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    interval,
    ray::Ray,
    utility::{INFINITY, random_float},
    vec3::{Point3, Vec3, random_on_hemisphere, unit_vector},
};

pub struct Camera {
    pub image_height: i32,
    pub image_width: i32,
    pub aspect_ratio: f64,
    pub center: Point3,
    pub pixel00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    pub max_depth: i32,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        eprintln!("{}, {}", self.image_height, self.image_width);
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let mut sample = 0;

                while sample < self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, world, self.max_depth);
                    sample += 1;
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        eprintln!("\rDone.           ");
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::new();
        if world.hit(
            ray,
            &interval::Interval {
                min: 0.001,
                max: INFINITY,
            },
            &mut rec,
        ) {
            let mut scattered: Ray = Ray::default();
            let mut attenuation = Color::default();

            if rec.mat.scatter(ray, &rec, &mut scattered, &mut attenuation) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn initialize(&mut self) {
        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 400;

        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_height: 0,
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 100,
            pixel_samples_scale: 1.0 / 10.0,
            max_depth: 10,
        }
    }
}
