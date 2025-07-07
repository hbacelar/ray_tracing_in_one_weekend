use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use ray_tracing_in_one_weekend::{
    camera::CameraBuilder,
    color::Color,
    material::{Dielectric, Lambertian, MaterialKind, Metal},
    sphere::Sphere,
    vec3::{Point, Vec3},
};
use rayon::prelude::*;

fn main() {
    // World
    let mut materials: Vec<MaterialKind> = Vec::new();

    let mut world_centers: Vec<Point> = Vec::new();

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat: f64 = rand::random();
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                world_centers.push(center);
                if chose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(&mut rand::rng()) * Vec3::random(&mut rand::rng());
                    materials.push(MaterialKind::Lambertian(Lambertian::new(albedo.into())));
                } else if chose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(&mut rand::rng(), 0.5, 1.0);
                    let fuz: f64 = rand::random_range(0.0..0.5);
                    materials.push(MaterialKind::Metal(Metal::new(albedo.into(), fuz)));
                } else {
                    // glass
                    materials.push(MaterialKind::Dielectric(Dielectric::new(1.5)));
                }
            }
        }
    }

    let mut world: Vec<Sphere<'_, MaterialKind>> = Vec::new();
    // ground
    let ground_material = MaterialKind::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    // big 3
    let glass = MaterialKind::Dielectric(Dielectric::new(1.5));
    world.push(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, &glass));
    let diffuse = MaterialKind::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, &diffuse));
    let metal = MaterialKind::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, &metal));

    //rest
    for (idx, center) in world_centers.iter().enumerate() {
        let mat = materials
            .get(idx)
            .expect("materials and centers are not the same size");
        world.push(Sphere::new(*center, 0.2, mat));
    }

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let cam = CameraBuilder::new()
        .image_width(image_width)
        .aspect_ratio(aspect_ratio)
        .samples_per_pixel(10)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Point::new(13.0, 2.0, 3.0))
        .lookat(Point::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    //render
    let n_passes = 10;
    let results: Vec<Vec<Color>> = (0..n_passes)
        .into_par_iter()
        .map(|_| {
            let mut rng = ChaCha12Rng::from_rng(&mut rand::rng());
            cam.render(&mut rng, &world)
        })
        .collect();

    // Average all runs
    let mut final_image = vec![Color::default(); (cam.image_width * cam.image_height) as usize];
    for image in results {
        for (i, pixel) in image.into_iter().enumerate() {
            final_image[i] += pixel;
        }
    }

    for pixel in &mut final_image {
        *pixel /= n_passes as f64;
    }
    Color::output_pixels(final_image, cam.image_width, cam.image_height);
}
