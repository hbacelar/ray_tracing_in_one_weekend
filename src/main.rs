use ray_tracing_in_one_weekend::{camera::Camera, sphere::Sphere, vec3::Point};

fn main() {
    // World
    let world = vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let cam = Camera::new(image_width, aspect_ratio, 10, 50);
    cam.render(&world);
}
