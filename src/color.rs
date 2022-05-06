use crate::vec::Vec3;
use std::io::Write;

pub fn write_color(mut writter: Box<dyn Write>, pixel_color: &Vec3) {
    let ir = (pixel_color.x() * 255.99).trunc();
    let ig = (pixel_color.y() * 255.99).trunc();
    let ib = (pixel_color.z() * 255.99).trunc();
    writeln!(writter, "{} {} {}", ir, ig, ib).unwrap();
}
