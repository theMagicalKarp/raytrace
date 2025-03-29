use crate::geometry::BvhNode;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::Quad;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

use super::Geometry;

#[derive(Debug, Clone)]
pub struct Cube {
    children: Box<BvhNode>,
}

impl Cube {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>, material: Material) -> Self {
        let min = Vector3::new(f32::min(a.x, b.x), f32::min(a.y, b.y), f32::min(a.z, b.z));
        let max = Vector3::new(f32::max(a.x, b.x), f32::max(a.y, b.y), f32::max(a.z, b.z));

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        let primitives = [
            Quad::geomtry(Vector3::new(min.x, min.y, max.z), dx, dy, material.clone()),
            Quad::geomtry(Vector3::new(max.x, min.y, max.z), -dz, dy, material.clone()),
            Quad::geomtry(Vector3::new(max.x, min.y, min.z), -dx, dy, material.clone()),
            Quad::geomtry(Vector3::new(min.x, min.y, min.z), dz, dy, material.clone()),
            Quad::geomtry(Vector3::new(min.x, max.y, max.z), dx, -dz, material.clone()),
            Quad::geomtry(Vector3::new(min.x, min.y, min.z), dx, dy, material.clone()),
        ];

        let children = Box::new(BvhNode::new(primitives.to_vec()));

        Cube { children }
    }
    pub fn geometry(a: Vector3<f32>, b: Vector3<f32>, material: Material) -> Geometry {
        Geometry::Cube(Cube::new(a, b, material))
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        self.children.hit(r, interval, record)
    }

    fn bounding_box(&self) -> Aabb {
        self.children.bounding_box()
    }
}
