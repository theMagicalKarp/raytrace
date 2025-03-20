use crate::interval::Interval;
use crate::material::Lambertian;
use crate::material::Material;
use crate::math;
use crate::ray::Ray;
use std::marker::Sync;
use std::sync::Arc;

pub struct HitRecord {
    pub point: math::Vector3<f32>,
    pub normal: math::Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: math::Vector3<f32>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn default() -> Self {
        HitRecord {
            point: math::Vector3::default(),
            normal: math::Vector3::default(),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian::new(math::Vector3::new(1.0, 0.75, 0.79))),
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: math::Vector3<f32>,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = (r.direction * r.direction).sum();
        let half_b = oc.dot(r.direction);
        let c = (oc * oc).sum() - self.radius * self.radius;

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
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        record.material = Arc::clone(&self.material);

        true
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = *interval;

        for object in &self.objects {
            if object.hit(r, &closest_so_far, record) {
                hit_anything = true;
                closest_so_far.max = record.t;
            }
        }

        hit_anything
    }
}
