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
    Dielectric(Dielectric),
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter<T>(&self, ray: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        // Small displace on sphere fuzz
        let reflected = ray.dir.reflect(&hit_record.normal) + (Vec3::random_unit() * self.fuzz);
        let scattered = Ray::new(hit_record.p, reflected);

        // check if ray reflect is wrong dir after fuzz
        if scattered.dir.dot(&hit_record.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter<T>(&self, ray_in: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray_in.dir.unit_vector();
        let refracted = unit_dir.refract(&hit_record.normal, ri);

        Some(Scatter {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_record.p, refracted),
        })
    }
}

impl Material for MaterialKind {
    fn scatter<T>(&self, ray_in: &Ray, hit_record: &HitRecord<T>) -> Option<Scatter> {
        match self {
            MaterialKind::Lambertian(mat) => mat.scatter(ray_in, hit_record),
            MaterialKind::Metal(mat) => mat.scatter(ray_in, hit_record),
            MaterialKind::Dielectric(mat) => mat.scatter(ray_in, hit_record),
        }
    }
}
