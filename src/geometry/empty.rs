use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Empty {}

impl Empty {
    pub fn geometry() -> Geometry {
        Geometry::Empty(Empty {})
    }
}

impl Hittable for Empty {
    fn hit(&self, _: &Ray, _: &Interval, _: &mut HitRecord, _: &mut ThreadRng) -> bool {
        false
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::default()
    }
}
