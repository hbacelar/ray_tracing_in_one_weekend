use rand::Rng;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Debug, Clone)]
pub struct CameraBuilder {
    image_width: u32,
    aspect_ratio: f64,
    samples_per_pixel: u32,
    max_depth: u32,

    vfov: f64,
    lookfrom: Point,
    lookat: Point,
    vup: Vec3,

    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 100,
            aspect_ratio: 1.0,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point::new(0.0, 0.0, 0.0),
            lookat: Point::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image_width(mut self, w: u32) -> Self {
        self.image_width = w;
        self
    }

    pub fn aspect_ratio(mut self, r: f64) -> Self {
        self.aspect_ratio = r;
        self
    }

    pub fn samples_per_pixel(mut self, spp: u32) -> Self {
        self.samples_per_pixel = spp;
        self
    }

    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn vfov(mut self, angle: f64) -> Self {
        self.vfov = angle;
        self
    }

    pub fn lookfrom(mut self, p: Point) -> Self {
        self.lookfrom = p;
        self
    }

    pub fn lookat(mut self, p: Point) -> Self {
        self.lookat = p;
        self
    }

    pub fn vup(mut self, v: Vec3) -> Self {
        self.vup = v;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: f64) -> Self {
        self.focus_dist = dist;
        self
    }

    pub fn build(self) -> Camera {
        let image_height = (self.image_width as f64 / self.aspect_ratio).max(1.0) as u32;

        let center = self.lookfrom;

        // Viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = self.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        eprintln!("Image: {}x{}", self.image_width, image_height);
        eprintln!(
            "Camera: center: {}, focus_dist: {}, viewport_height: {}, viewport_width: {}",
            center, self.focus_dist, viewport_height, viewport_width
        );
        eprintln!(
            "ViewPort: viewport_u: {}, view_port_v: {}, pixel_delta_u: {}, pixel_delta_v: {}",
            viewport_u, viewport_v, pixel_delta_u, pixel_delta_v
        );

        Camera {
            image_width: self.image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            pixel_samples_scale,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    // Count of random samples for each pixel
    samples_per_pixel: u32,
    // Maximum number of ray bounces into scene
    max_depth: u32,

    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

fn ray_color<M: Material, T: Hittable<M>>(
    rng: &mut impl Rng,
    ray: &Ray,
    depth: u32,
    world: &[T],
) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3::default();
    }
    if let Some(h) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        if let Some(s) = h.material.scatter(rng, ray, &h) {
            return *s.attenuation * ray_color(rng, &s.scattered, depth - 1, world);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    white * (1.0 - a) + blue * a
}

fn sample_square(rng: &mut impl Rng) -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    Vec3 {
        x: rng.random::<f64>() - 0.5,
        y: rng.random::<f64>() - 0.5,
        z: 0.0,
    }
}

impl Camera {
    fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk(rng);
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn get_ray(&self, rng: &mut impl Rng, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };

        Ray {
            dir: pixel_sample - ray_origin,
            origin: ray_origin,
        }
    }

    pub fn render<M: Material, T: Hittable<M>>(
        &self,
        rng: &mut impl Rng,
        world: &[T],
    ) -> Vec<Color> {
        let mut pixels = Vec::with_capacity((self.image_width * self.image_height) as usize);
        // print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);

            for i in 0..self.image_width {
                let mut color = Vec3::default();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(rng, i, j);
                    color += ray_color(rng, &ray, self.max_depth, world);
                }
                let color: Color = (color * self.pixel_samples_scale).into();
                pixels.push(color);

                // println!("{color}");
            }
        }
        eprint!("\rDone.                   \n");
        pixels
    }
}
