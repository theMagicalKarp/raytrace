use crate::material::Material;
use crate::math;
use crate::math::near_zero;
use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + math::random_normal();
        if near_zero(&scatter_direction) {
            scatter_direction = record.normal;
        }

        scattered.origin = record.point;
        scattered.time = r_in.time;
        scattered.direction = scatter_direction;
        attenuation.copy_from(&self.albedo);
        true
    }
}
