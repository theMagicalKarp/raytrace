use crate::interval::Interval;
use crate::material::Lambertian;
use crate::material::Material;
use crate::object::aabb::Aabb;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::marker::Sync;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn default() -> Self {
        HitRecord {
            point: Vector3::default(),
            normal: Vector3::default(),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian::new(Vector3::new(1.0, 0.75, 0.79))),
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}
