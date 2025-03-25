use crate::material::Material;
use crate::math;
use crate::math::reflect;
use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, roughness: f32) -> Self {
        Metal { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(&r_in.direction, &record.normal);
        reflected = reflected.normalize() + (math::random_normal() * self.roughness);

        scattered.origin = record.point;
        scattered.time = r_in.time;
        scattered.direction = reflected;
        attenuation.copy_from(&self.albedo);
        scattered.direction.dot(&record.normal) > 0.0
    }
}
