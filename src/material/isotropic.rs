use crate::geometry::HitRecord;
use crate::material::Material;
use crate::material::Surface;
use crate::material::texture::Sample;
use crate::material::texture::Texture;
use crate::math;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Isotropic {
    pub texture: Texture,
}

impl Isotropic {
    pub fn new(texture: Texture) -> Isotropic {
        Isotropic { texture }
    }
    pub fn material(texture: Texture) -> Material {
        Material::Isotropic(Isotropic::new(texture))
    }
}

impl Surface for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        scattered.origin = record.point;
        scattered.time = r_in.time;
        scattered.direction = math::random_normal();
        attenuation.copy_from(&self.texture.sample(record.u, record.v, record.point));
        true
    }

    fn emitted(&self, _: f32, _: f32, _: Vector3<f32>) -> Vector3<f32> {
        Vector3::<f32>::default()
    }
}
