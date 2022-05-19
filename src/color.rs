use crate::vec::Vec3;
use num;
use std::io::Write;

pub fn write_color(mut writter: Box<dyn Write>, pixel_color: &Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let ir = (num::clamp(r, 0.0, 0.999) * 256.0).trunc();
    let ig = (num::clamp(g, 0.0, 0.999) * 256.0).trunc();
    let ib = (num::clamp(b, 0.0, 0.999) * 256.0).trunc();
    writeln!(writter, "{} {} {}", ir, ig, ib).unwrap();
}
