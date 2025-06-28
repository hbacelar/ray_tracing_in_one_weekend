use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
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
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(100, 1.0)
    }
}

fn ray_color<T: Hittable>(ray: &Ray, world: &[T]) -> Color {
    if let Some(h) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        let normal = h.normal;
        let color: Color = (Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5).into();
        return color;
    }

    let unit_direction = ray.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let blended = white * (1.0 - a) + blue * a;

    blended.into()
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
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
        }
    }

    pub fn render<T: Hittable>(&self, world: &[T]) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);

            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);
                let color = ray_color(&ray, world);

                println!("{color}");
            }
        }
        eprint!("\rDone.                   \n");
    }
}
