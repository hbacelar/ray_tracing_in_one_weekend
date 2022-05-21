use num::traits::Pow;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{self, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(vec::unit_vec(*r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector(rng) * self.fuzz);

        if vec::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // schlink approximation
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).pow(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = vec::unit_vec(*r_in.direction());

        //Check if it can reflect or refract
        let cos_theta = vec::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            let direction = Vec3::reflect(unit_direction, rec.normal);
            Some((Ray::new(rec.p, direction), Vec3(1.0, 1.0, 1.0)))
        } else {
            let direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
            Some((Ray::new(rec.p, direction), Vec3(1.0, 1.0, 1.0)))
        }
    }
}
