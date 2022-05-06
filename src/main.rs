use std::io::{self, Write};

mod color;
mod ray;
mod vec;

use ray::Ray;
use vec::Vec3;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = vec::unit_vec(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    let image_height: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).round() as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = (ASPECT_RATIO * viewport_height).round();
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}\n255", IMAGE_WIDTH, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&r);
            color::write_color(Box::new(io::stdout()), &color);
        }
    }

    eprintln!("\nDone!");
}
