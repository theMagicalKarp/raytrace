use crate::geometry::aabb::Aabb;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
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
#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_axis_as_index() {
        assert_eq!(Axis::X.as_index(), 0);
        assert_eq!(Axis::Y.as_index(), 1);
        assert_eq!(Axis::Z.as_index(), 2);
    }

    #[test]
    fn test_axis_compare_bboxes() {
        let a = Aabb::from_points(Vector3::new(1.0, 2.0, 3.0), Vector3::new(2.0, 3.0, 4.0));
        let b = Aabb::from_points(Vector3::new(0.0, -2.0, 10.0), Vector3::new(2.0, 5.0, 11.0));

        assert_eq!(
            Axis::X.compare_bboxes(&a, &b),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            Axis::Y.compare_bboxes(&a, &b),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            Axis::Z.compare_bboxes(&a, &b),
            Some(std::cmp::Ordering::Less)
        );
    }
}
