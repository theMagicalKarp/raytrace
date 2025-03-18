#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    // pub fn empty() -> Self {
    //     Self { min: f32::INFINITY, max: f32::NEG_INFINITY }
    // }

    // pub fn universe() -> Self {
    //     Self { min: f32::NEG_INFINITY, max: f32::INFINITY }
    // }

    // pub fn size(&self) -> f32 {
    //     self.max-self.min
    // }

    // pub fn contains(&self, value: f32) -> bool {
    //     value >= self.min && value <= self.max
    // }

    pub fn surrounds(&self, value: f32) -> bool {
        value > self.min && value < self.max
    }
}
