use ray_tracing_in_one_weekend::{
    camera::Camera,
    color::Color,
    material::{Dielectric, Lambertian, MaterialKind, Metal},
    sphere::Sphere,
    vec3::{Point, Vec3},
};

fn main() {
    // World

    let mat_ground = MaterialKind::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = MaterialKind::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = MaterialKind::Dielectric(Dielectric::new(1.50));
    let mat_bubble = MaterialKind::Dielectric(Dielectric::new(1.00 / 1.50));
    let mat_right = MaterialKind::Metal(Metal::new(Color::new(0.8, 0.5, 0.2), 1.0));

    let world = vec![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, &mat_ground),
        Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, &mat_center),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, &mat_left),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.4, &mat_bubble),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, &mat_right),
    ];

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 800;

    let cam = Camera::new(
        image_width,
        aspect_ratio,
        100,
        50,
        20.0,
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        3.4,
    );
    cam.render(&world);
}
