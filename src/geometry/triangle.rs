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
pub struct Vertex {
    pub position: Vector3<f64>,
    pub normal: Option<Vector3<f64>>,
}

impl Vertex {
    pub fn new(position: Vector3<f64>, normal: Option<Vector3<f64>>) -> Self {
        Vertex { position, normal }
    }

    pub fn from_poligon(position: (f32, f32, f32, f32), normal: Option<(f32, f32, f32)>) -> Self {
        let normal = normal.map(|n| Vector3::new(n.0 as f64, n.1 as f64, n.2 as f64));
        let position = Vector3::new(position.0 as f64, position.1 as f64, position.2 as f64);
        Vertex { position, normal }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
    pub bbox: Aabb,
    pub material: Material,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex, material: Material) -> Triangle {
        let a_to_b = Aabb::from_points(a.position, b.position);
        let b_to_c = Aabb::from_points(b.position, c.position);
        let c_to_a = Aabb::from_points(c.position, a.position);
        let bbox = Aabb::from_boxes(&Aabb::from_boxes(&a_to_b, &b_to_c), &c_to_a);
        Triangle {
            a,
            b,
            c,
            material,
            bbox,
        }
    }

    pub fn geometry(a: Vertex, b: Vertex, c: Vertex, material: Material) -> Geometry {
        Geometry::Triangle(Triangle::new(a, b, c, material))
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord, _: &mut ThreadRng) -> bool {
        let e1 = self.b.position - self.a.position;
        let e2 = self.c.position - self.a.position;

        let ray_cross_e2 = r.direction.cross(&e2);
        let determinant = e1.dot(&ray_cross_e2);
        if determinant.abs() < f64::EPSILON {
            return false; // Ray is parallel to the triangle
        }

        let inv_determinant = 1.0 / determinant;
        let s = r.origin - self.a.position;

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

        let outward_normal = match (self.a.normal, self.b.normal, self.c.normal) {
            (Some(a_normal), Some(b_normal), Some(c_normal)) => {
                ((1.0 - u - v) * a_normal + u * b_normal + v * c_normal).normalize()
            }
            _ => e1.cross(&e2).normalize(),
        };

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
