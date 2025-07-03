use ray_tracing_in_one_weekend::{
    camera::Camera, color::Color, material::Lambertian, sphere::Sphere, vec3::Point,
};

fn main() {
    // World
    let mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    let world = vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, &mat),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, &mat),
    ];

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let cam = Camera::new(image_width, aspect_ratio, 10, 50);
    cam.render(&world);
}
