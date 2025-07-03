use ray_tracing_in_one_weekend::{
    camera::Camera,
    color::Color,
    material::{Lambertian, MaterialKind, Metal},
    sphere::Sphere,
    vec3::Point,
};

fn main() {
    // World
    let mat_ground = MaterialKind::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_left = MaterialKind::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_center = MaterialKind::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_right = MaterialKind::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let world = vec![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, &mat_ground),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, &material_left),
        Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, &material_center),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, &material_right),
    ];

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let cam = Camera::new(image_width, aspect_ratio, 10, 50);
    cam.render(&world);
}
