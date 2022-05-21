use std::io::{self, Write};

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec;

use rand::prelude::ThreadRng;
use ray::Ray;
use vec::Vec3;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use rand::distributions::{Distribution, Uniform};

fn ray_color(r: &Ray, world: &HittableList, depth: u8, rng: &mut ThreadRng) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit, rng) {
            return attenuation * ray_color(&scattered, world, depth - 1, rng);
        }
        return Vec3(0.0, 0.0, 0.0);
    }

    let unit_direction = vec::unit_vec(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_f64 = Uniform::<f64>::from(0.0..1.0);

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    let image_height: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).round() as i32;
    let samples_per_pixel = 100;
    const MAX_DEPTH: u8 = 50;

    // World

    let material_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3(0.7, 0.3, 0.3));
    let material_left = Metal::new(Vec3(0.8, 0.8, 0.8));
    let material_right = Metal::new(Vec3(0.8, 0.6, 0.2));

    let sphere_ground = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material_ground),
    };
    let sphere_center = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_center),
    };
    let sphere_left = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_left),
    };
    let sphere_right = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_right),
    };

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(sphere_ground),
        Box::new(sphere_center),
        Box::new(sphere_left),
        Box::new(sphere_right),
    ];
    let world = HittableList::new(objects);

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3");
    println!("{} {}\n255", IMAGE_WIDTH, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64.sample(&mut rng)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64.sample(&mut rng)) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH, &mut rng);
            }
            color::write_color(Box::new(io::stdout()), &pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone!");
}
