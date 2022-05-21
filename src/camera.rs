use std::f64::consts::PI;

use crate::{
    ray::Ray,
    vec::{self, Vec3},
};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = (aspect_ratio * viewport_height).round();

        let w = vec::unit_vec(lookfrom - lookat);
        let u = vec::unit_vec(vec::cross(&vup, &w));
        let v = vec::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
