use crate::geometry::HitRecord;
use crate::material::Material;
use crate::material::Surface;
use crate::math::reflect;
use crate::math::reflectance;
use crate::math::refract;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn material(refraction_index: f64) -> Material {
        Material::Dielectric(Dielectric { refraction_index })
    }
}

impl Surface for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        attenuation.copy_from(&Vector3::from_element(1.0));
        let r_index = match record.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let normalized_direction = r_in.direction.normalize();

        let cos_theta = f64::min((-normalized_direction).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = r_index * sin_theta > 1.0;

        scattered.origin = record.point;
        scattered.time = r_in.time;

        scattered.direction =
            match cannot_refract || (reflectance(cos_theta, r_index) > rng.random::<f64>()) {
                true => reflect(&normalized_direction, &record.normal),
                false => refract(&normalized_direction, &record.normal, r_index),
            };

        true
    }

    fn emitted(&self, _: f64, _: f64, _: Vector3<f64>) -> Vector3<f64> {
        Vector3::<f64>::default()
    }
}
