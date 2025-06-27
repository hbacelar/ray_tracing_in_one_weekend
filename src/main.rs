use ray_tracing_in_one_weekend::{
    color::Color,
    ray::Ray,
    vec3::{Point, Vec3},
};

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = center - ray.origin;
    let a = ray.dir.dot(&ray.dir);
    let b = (ray.dir * -2.0).dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;

    if discriminant < 0.0 {
        None
    } else {
        let sqrt_d = discriminant.sqrt();
        Some((-b - sqrt_d) / (2.0 * a))
    }
}

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point::new(0.0, 0.0, -1.0);
    if let Some(t) = hit_sphere(sphere_center, 0.5, ray) {
        let normal = (ray.at(t) - sphere_center).unit_vector();

        let color: Color = (Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5).into();
        return color;
    }

    let unit_direction = ray.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let blended = white * (1.0 - a) + blue * a;

    blended.into()
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as u32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    eprintln!("Image: {}x{}", image_width, image_height);
    eprintln!(
        "Camera: center: {}, focal_length: {}, viewport_height: {}, viewport_width: {}",
        camera_center, focal_length, viewport_height, viewport_width
    );
    eprintln!(
        "ViewPort: viewport_u: {}, view_port_v: {}, pixel_delta_u: {}, pixel_delta_v: {}",
        viewport_u, viewport_v, pixel_delta_u, pixel_delta_v
    );

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);

            println!("{color}");
        }
    }
    eprint!("\rDone.                   \n");
}
