use crate::geometry::aabb::Aabb;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Axis {
    #[serde(rename = "x")]
    X = 0,
    #[serde(rename = "y")]
    Y = 1,
    #[serde(rename = "z")]
    Z = 2,
}

impl Axis {
    pub fn as_index(&self) -> usize {
        self.clone() as usize
    }

    pub fn compare_bboxes(&self, a: &Aabb, b: &Aabb) -> Option<std::cmp::Ordering> {
        match self {
            Axis::X => a.x.partial_cmp(&b.x),
            Axis::Y => a.y.partial_cmp(&b.y),
            Axis::Z => a.z.partial_cmp(&b.z),
        }
    }
}
