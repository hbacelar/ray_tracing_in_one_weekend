use crate::{
    ray::Ray,
    vec::{self, Vec3},
};

#[derive(Debug)]
pub struct HitRecord {
    p: Vec3,
    pub normal: Vec3,
    t: f64,
    front_face: bool,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HitRecord {
    pub fn new(p: Vec3, outward_normal: Vec3, r: &Ray, t: f64) -> Self {
        let front_face = vec::dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in &self.objects {
            match object.hit(r, t_min, closest) {
                None => continue,
                Some(h) => {
                    closest = h.t;
                    best_hit = Some(h);
                }
            }
        }
        best_hit
    }
}
