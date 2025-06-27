use crate::{
    hittable::{HitRecord, Hittable},
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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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

        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrt_d) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let p = ray.at(root);

        Some(HitRecord {
            p,
            normal: (p - self.center) / self.radius,
            t: root,
        })
    }
}
