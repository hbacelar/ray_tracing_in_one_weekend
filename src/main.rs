use std::io::{self, Write};

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3");
    println!("{} {} 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir = (r * 255.999).trunc();
            let ig = (g * 255.999).trunc();
            let ib = (b * 255.999).trunc();

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone!");
}
