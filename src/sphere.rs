use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Point,
};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let root = (h - sqrt_d) / a;

        if !ray_t.surrounds(root) {
            let root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, p, root, outward_normal))
    }
}
