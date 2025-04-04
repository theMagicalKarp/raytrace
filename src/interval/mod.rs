use core::f64;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn universe() -> Self {
        let min = f64::NEG_INFINITY;
        let max = f64::INFINITY;
        Interval { min, max }
    }

    pub fn combine(a: Interval, b: Interval) -> Interval {
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        let min = f64::INFINITY;
        let max = f64::NEG_INFINITY;
        Interval { min, max }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.min < other.min {
            Some(std::cmp::Ordering::Less)
        } else if self.min > other.min {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Add<f64> for Interval {
    type Output = Self;
    fn add(self, offset: f64) -> Self::Output {
        Interval {
            min: self.min + offset,
            max: self.max + offset,
        }
    }
}
