use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    // Count of random samples for each pixel
    samples_per_pixel: u32,
    // Maximum number of ray bounces into scene
    max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(100, 1.0, 10, 10)
    }
}

fn ray_color<M: Material, T: Hittable<M>>(ray: &Ray, depth: u32, world: &[T]) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3::default();
    }
    if let Some(h) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        if let Some(s) = h.material.scatter(ray, &h) {
            return *s.attenuation * ray_color(&s.scattered, depth - 1, world);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    white * (1.0 - a) + blue * a
}

fn sample_square() -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    Vec3 {
        x: rand::random::<f64>() - 0.5,
        y: rand::random::<f64>() + 0.5,
        z: 0.0,
    }
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio).max(1.0) as u32;

        let center = Point::new(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        eprintln!("Image: {}x{}", image_width, image_height);
        eprintln!(
            "Camera: center: {}, focal_length: {}, viewport_height: {}, viewport_width: {}",
            center, focal_length, viewport_height, viewport_width
        );
        eprintln!(
            "ViewPort: viewport_u: {}, view_port_v: {}, pixel_delta_u: {}, pixel_delta_v: {}",
            viewport_u, viewport_v, pixel_delta_u, pixel_delta_v
        );

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        Ray {
            dir: pixel_sample - self.center,
            origin: self.center,
        }
    }

    pub fn render<M: Material, T: Hittable<M>>(&self, world: &[T]) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);

            for i in 0..self.image_width {
                let mut color = Vec3::default();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += ray_color(&ray, self.max_depth, world);
                }
                let color: Color = (color * self.pixel_samples_scale).into();

                println!("{color}");
            }
        }
        eprint!("\rDone.                   \n");
    }
}
