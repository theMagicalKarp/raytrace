use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    pub material: Material,
    pub bbox: Aabb,
}

pub fn get_sphere_uv(point: Vector3<f32>) -> (f32, f32) {
    let theta = point.y.acos();
    let phi = (-point.z).atan2(point.x) + PI;

    (phi / (2.0 * PI), theta / PI)
}

impl Sphere {
    pub fn new(
        center: Vector3<f32>,
        direction: Vector3<f32>,
        radius: f32,
        material: Material,
    ) -> Sphere {
        let center = Ray::new(center, direction, 0.0);
        let rvec = Vector3::from_element(radius);
        let bbox = Aabb::from_boxes(
            &Aabb::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec),
            &Aabb::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec),
        );
        Sphere {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time);
        let oc = r.origin - current_center;

        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.point = r.at(root);
        let outward_normal = (record.point - current_center) / self.radius;
        record.set_face_normal(r, &outward_normal);
        record.material = self.material.clone();
        (record.u, record.v) = get_sphere_uv(outward_normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
