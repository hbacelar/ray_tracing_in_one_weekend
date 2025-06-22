fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_height {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b: f64 = 0.0;

            let ir = (255.9 * r) as u8;
            let ig = (255.9 * g) as u8;
            let ib = (255.0 * b) as u8;

            println!("{ir} {ig} {ib}");
        }
    }
    eprint!("\rDone.                   \n");
}
