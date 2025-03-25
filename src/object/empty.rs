use crate::interval::Interval;
use crate::object::aabb::Aabb;
use crate::object::hittable::HitRecord;
use crate::object::hittable::Hittable;
use crate::ray::Ray;

pub struct Empty {}

impl Hittable for Empty {
    fn hit(&self, _: &Ray, _: &Interval, _: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::default()
    }
}
