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
        } else if self.max < other.max {
            Some(std::cmp::Ordering::Less)
        } else if self.max > other.max {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_new() {
        let interval = Interval::new(1.0, 5.0);
        assert_eq!(interval.min, 1.0);
        assert_eq!(interval.max, 5.0);
    }

    #[test]
    fn test_interval_universe() {
        let interval = Interval::universe();
        assert_eq!(interval.min, f64::NEG_INFINITY);
        assert_eq!(interval.max, f64::INFINITY);
    }

    #[test]
    fn test_interval_combine() {
        let a = Interval::new(1.0, 5.0);
        let b = Interval::new(3.0, 7.0);
        let combined = Interval::combine(a, b);
        assert_eq!(combined.min, 1.0);
        assert_eq!(combined.max, 7.0);
    }

    #[test]
    fn test_interval_size() {
        let interval = Interval::new(1.0, 5.0);
        assert_eq!(interval.size(), 4.0);
    }

    #[test]
    fn test_interval_expand() {
        let interval = Interval::new(2.0, 6.0);
        let expanded = interval.expand(4.0);
        assert_eq!(expanded.min, 0.0);
        assert_eq!(expanded.max, 8.0);
    }

    #[test]
    fn test_interval_contains() {
        let interval = Interval::new(1.0, 5.0);
        assert!(interval.contains(3.0));
        assert!(!interval.contains(0.0));
        assert!(!interval.contains(6.0));
    }

    #[test]
    fn test_interval_clamp() {
        let interval = Interval::new(1.0, 5.0);
        assert_eq!(interval.clamp(3.0), 3.0);
        assert_eq!(interval.clamp(0.0), 1.0);
        assert_eq!(interval.clamp(6.0), 5.0);
    }

    #[test]
    fn test_interval_surrounds() {
        let interval = Interval::new(1.0, 5.0);
        assert!(interval.surrounds(3.0));
        assert!(!interval.surrounds(1.0));
        assert!(!interval.surrounds(5.0));
    }

    #[test]
    fn test_interval_default() {
        let interval = Interval::default();
        assert_eq!(interval.min, f64::INFINITY);
        assert_eq!(interval.max, f64::NEG_INFINITY);
    }

    #[test]
    fn test_interval_partial_eq() {
        let a = Interval::new(1.0, 5.0);
        let b = Interval::new(1.0, 5.0);
        let c = Interval::new(2.0, 6.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_interval_partial_ord() {
        let a = Interval::new(1.0, 5.0);
        let b = Interval::new(2.0, 6.0);
        let c = Interval::new(1.0, 5.0);
        let d = Interval::new(1.0, 4.0);
        assert!(a < b);
        assert!(b > a);
        assert!(a == c);
        assert!(a > d);
        assert!(d < a);
    }

    #[test]
    fn test_interval_add() {
        let interval = Interval::new(1.0, 5.0);
        let result = interval + 2.0;
        assert_eq!(result.min, 3.0);
        assert_eq!(result.max, 7.0);
    }
}
