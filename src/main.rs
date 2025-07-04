use std::f64::consts::PI;

use ray_tracing_in_one_weekend::{
    camera::Camera,
    color::Color,
    material::{Lambertian, MaterialKind},
    sphere::Sphere,
    vec3::Point,
};

fn main() {
    let r = (PI / 4.0).cos();
    // World
    let mat_left = MaterialKind::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = MaterialKind::Lambertian(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let world = vec![
        Sphere::new(Point::new(-r, 0.0, -1.0), r, &mat_left),
        Sphere::new(Point::new(r, 0.0, -1.0), r, &mat_right),
    ];

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 800;

    let cam = Camera::new(image_width, aspect_ratio, 10, 50, 90.0);
    cam.render(&world);
}
