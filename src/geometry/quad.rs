use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Quad {
    pub q: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub material: Material,
    pub bbox: Aabb,
    pub normal: Vector3<f64>,
    pub d: f64,
    pub w: Vector3<f64>,
}

impl Quad {
    pub fn new(q: Vector3<f64>, u: Vector3<f64>, v: Vector3<f64>, material: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / (n.dot(&n));

        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
        let bbox = Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2);
        Quad {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            d,
            w,
        }
    }
    pub fn geometry(
        q: Vector3<f64>,
        u: Vector3<f64>,
        v: Vector3<f64>,
        material: Material,
    ) -> Geometry {
        Geometry::Quad(Quad::new(q, u, v, material))
    }
}

fn is_interior(a: f64, b: f64, record: &mut HitRecord) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }

    record.u = a;
    record.v = b;
    true
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord, _: &mut ThreadRng) -> bool {
        let denom = self.normal.dot(&r.direction);
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(&r.origin)) / denom;

        if !interval.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !is_interior(alpha, beta, record) {
            return false;
        }

        record.t = t;
        record.point = intersection;
        record.material = self.material.clone();
        record.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
