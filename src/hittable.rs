use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point, t: f64, outward_normal: Vec3) -> Self {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for &[T] {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for hittable in self.iter() {
            if let Some(hit_record) = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                hit = Some(hit_record);
            }
        }

        hit
    }
}
