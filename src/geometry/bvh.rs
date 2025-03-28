use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::geometry::empty::Empty;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct BvhNode {
    left: Box<Geometry>,
    right: Box<Geometry>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn geometry(objects: Vec<Geometry>) -> Geometry {
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

        let left: Geometry = match left_objects.len() {
            0 => Empty::geometry(),
            1 => left_objects[0].clone(),
            _ => BvhNode::geometry(left_objects.to_vec()),
        };
        let left = Box::new(left);

        let right: Geometry = match right_objects.len() {
            0 => Empty::geometry(),
            1 => right_objects[0].clone(),
            _ => BvhNode::geometry(right_objects.to_vec()),
        };
        let right = Box::new(right);

        Geometry::BvhNode(BvhNode { left, right, bbox })
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
