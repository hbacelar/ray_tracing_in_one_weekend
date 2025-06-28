use std::{fmt, ops::Deref};

use crate::{interval::Interval, vec3::Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub Vec3);

const INTENSITY: Interval = Interval::new(0.000, 0.999);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r_byte = (256.0 * INTENSITY.clamp(self.0.x)) as u8;
        let g_byte = (256.0 * INTENSITY.clamp(self.0.y)) as u8;
        let b_byte = (256.0 * INTENSITY.clamp(self.0.z)) as u8;

        write!(f, "{} {} {}", r_byte, g_byte, b_byte)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
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
