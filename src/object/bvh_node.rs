use crate::interval::Interval;
use crate::object::aabb::Aabb;
use crate::object::empty::Empty;
use crate::object::hittable::HitRecord;
use crate::object::hittable::Hittable;
use crate::ray::Ray;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        let mut bbox = Aabb::default();
        for object in objects.iter() {
            bbox = Aabb::from_boxes(&bbox, &object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let mut objects = objects;
        let object_span = objects.len();
        let mid = object_span / 2;

        objects.sort_by(|a, b| {
            axis.compare_bboxes(&a.bounding_box(), &b.bounding_box())
                .unwrap()
        });

        let (left_objects, right_objects) = objects.split_at(mid);

        let left: Arc<dyn Hittable> = match left_objects.len() {
            0 => Arc::new(Empty {}),
            1 => left_objects[0].clone(),
            _ => Arc::new(BvhNode::new(left_objects.to_vec())),
        };

        let right: Arc<dyn Hittable> = match right_objects.len() {
            0 => Arc::new(Empty {}),
            1 => right_objects[0].clone(),
            _ => Arc::new(BvhNode::new(right_objects.to_vec())),
        };

        BvhNode { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, interval) {
            return false;
        }

        let left_hit = self.left.hit(r, interval, record);
        let new_interval = match left_hit {
            true => &Interval {
                min: interval.min,
                max: record.t,
            },
            false => interval,
        };

        let right_hit = self.right.hit(r, new_interval, record);

        left_hit || right_hit
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
