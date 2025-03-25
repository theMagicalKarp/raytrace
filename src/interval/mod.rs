#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn combine(a: Interval, b: Interval) -> Interval {
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        Interval { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    pub fn surrounds(&self, value: f32) -> bool {
        value > self.min && value < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        let min = f32::INFINITY;
        let max = f32::NEG_INFINITY;
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
