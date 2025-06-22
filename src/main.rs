use ray_tracing_in_one_weekend::color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_height {
            let color = Color::new(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.0,
            );
            println!("{color}");
        }
    }
    eprint!("\rDone.                   \n");
}
