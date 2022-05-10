use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec::{self, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

// Math: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().len_squared();
        let half_b = vec::dot(&oc, r.direction());
        let c = oc.len_squared() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);

        Some(HitRecord {
            t: root,
            p,
            normal: (p - self.center) / self.radius,
        })
    }
}
