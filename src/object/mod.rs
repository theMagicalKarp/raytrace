use crate::math;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: math::Vector3<f32>,
    pub normal: math::Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
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
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: math::Vector3<f32>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
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
        if root < ray_tmin || ray_tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < ray_tmin || ray_tmax < root {
                return false;
            }
        }

        record.t = root;
        record.point = r.at(root);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(&r, outward_normal);
        record.normal = (record.point - self.center) / self.radius;

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
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        hit_anything
    }
}
