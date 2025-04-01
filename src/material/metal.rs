use crate::geometry::HitRecord;
use crate::material::Material;
use crate::material::Surface;
use crate::math;
use crate::math::reflect;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vector3<f64>,
    pub roughness: f64,
}

impl Metal {
    pub fn material(albedo: Vector3<f64>, roughness: f64) -> Material {
        Material::Metal(Metal { albedo, roughness })
    }
}

impl Surface for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f64>,
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

    fn emitted(&self, _: f64, _: f64, _: Vector3<f64>) -> Vector3<f64> {
        Vector3::<f64>::default()
    }
}
