#[derive(Debug, Clone, Copy, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_enclose_both(a: &Interval, b: &Interval) -> Self {
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);

        Self { min, max }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max - padding,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
