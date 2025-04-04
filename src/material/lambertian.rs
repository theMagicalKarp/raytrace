use crate::geometry::HitRecord;
use crate::material::Material;
use crate::material::Surface;
use crate::material::texture::Sample;
use crate::material::texture::Texture;
use crate::math;
use crate::math::near_zero;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn material(texture: Texture) -> Material {
        Material::Lambertian(Lambertian { texture })
    }
}

impl Surface for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut scatter_direction = record.normal + math::random_normal(rng);
        if near_zero(&scatter_direction) {
            scatter_direction = record.normal;
        }

        scattered.origin = record.point;
        scattered.time = r_in.time;
        scattered.direction = scatter_direction;
        attenuation.copy_from(&self.texture.sample(record.u, record.v, record.point));
        true
    }

    fn emitted(&self, _: f64, _: f64, _: Vector3<f64>) -> Vector3<f64> {
        Vector3::<f64>::default()
    }
}
