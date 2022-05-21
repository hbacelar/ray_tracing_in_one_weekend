use std::f64::consts::PI;

use crate::{ray::Ray, vec::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = (aspect_ratio * viewport_height).round();
        let focal_length = 1.0;

        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
