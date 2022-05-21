use crate::{
    material::Material,
    ray::Ray,
    vec::{self, Vec3},
};

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    t: f64,
    front_face: bool,
    pub material: &'a dyn Material,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, outward_normal: Vec3, r: &Ray, t: f64, material: &'a dyn Material) -> Self {
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
            material,
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
