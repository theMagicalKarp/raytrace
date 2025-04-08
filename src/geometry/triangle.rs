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
pub struct Triangle {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
    pub bbox: Aabb,
    pub material: Material,
}

impl Triangle {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>, material: Material) -> Triangle {
        let a_to_b = Aabb::from_points(a, b);
        let b_to_c = Aabb::from_points(b, c);
        let c_to_a = Aabb::from_points(c, a);
        let bbox = Aabb::from_boxes(&Aabb::from_boxes(&a_to_b, &b_to_c), &c_to_a);
        Triangle {
            a,
            b,
            c,
            material,
            bbox,
        }
    }

    pub fn geometry(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        material: Material,
    ) -> Geometry {
        Geometry::Triangle(Triangle::new(a, b, c, material))
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord, _: &mut ThreadRng) -> bool {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = r.direction.cross(&e2);
        let determinant = e1.dot(&ray_cross_e2);
        if determinant.abs() < f64::EPSILON {
            return false; // Ray is parallel to the triangle
        }

        let inv_determinant = 1.0 / determinant;
        let s = r.origin - self.a;

        let u = inv_determinant * s.dot(&ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return false; // Intersection is outside the triangle
        }

        let t_cross_e1 = s.cross(&e1);
        let v = inv_determinant * r.direction.dot(&t_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return false; // Intersection is outside the triangle
        }

        let t = inv_determinant * e2.dot(&t_cross_e1);
        if t <= f64::EPSILON || !interval.contains(t) {
            return false;
        }

        let outward_normal = e1.cross(&e2).normalize();
        record.t = t;
        record.point = r.at(t);
        record.set_face_normal(r, &outward_normal);
        record.material = self.material.clone();
        record.u = u;
        record.v = v;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
