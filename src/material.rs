use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter<T>(&self, ray_in: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter>;
}

#[derive(Debug, Clone)]
pub enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter<T>(&self, _: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        let mut scatter_dir = hit_record.normal + Vec3::random_unit();

        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal
        }

        let scattered = Ray::new(hit_record.p, scatter_dir);
        Some(Scatter {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter<T>(&self, ray: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        let reflected = ray.dir.reflect(&hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);

        Some(Scatter {
            attenuation: self.albedo,
            scattered,
        })
    }
}

impl Material for MaterialKind {
    fn scatter<T>(&self, ray_in: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        match self {
            MaterialKind::Lambertian(mat) => mat.scatter(ray_in, hit_record),
            MaterialKind::Metal(mat) => mat.scatter(ray_in, hit_record),
        }
    }
}
