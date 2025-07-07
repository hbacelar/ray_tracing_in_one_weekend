use std::{
    fmt,
    ops::{AddAssign, Deref, DivAssign},
};

use crate::{interval::Interval, vec3::Vec3};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Color(pub Vec3);

const INTENSITY: Interval = Interval::new(0.000, 0.999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = linear_to_gamma(self.0.x);
        let g = linear_to_gamma(self.0.y);
        let b = linear_to_gamma(self.0.z);

        let r_byte = (256.0 * INTENSITY.clamp(r)) as u8;
        let g_byte = (256.0 * INTENSITY.clamp(g)) as u8;
        let b_byte = (256.0 * INTENSITY.clamp(b)) as u8;

        write!(f, "{} {} {}", r_byte, g_byte, b_byte)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn output_pixels(pixels: Vec<Color>, width: u32, height: u32) {
        print!("P3\n{} {}\n255\n", width, height);
        for color in pixels {
            println!("{color}");
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color::new(value.x, value.y, value.z)
    }
}

impl Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
