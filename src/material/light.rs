use crate::geometry::HitRecord;
use crate::material::Material;
use crate::material::Surface;
use crate::material::texture::Sample;
use crate::material::texture::Texture;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Light {
    pub texture: Texture,
}

impl Light {
    pub fn material(texture: Texture) -> Material {
        Material::Light(Light { texture })
    }
}

impl Surface for Light {
    fn scatter(
        &self,
        _: &Ray,
        _: &HitRecord,
        _: &mut Vector3<f64>,
        _: &mut Ray,
        _: &mut ThreadRng,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        self.texture.sample(u, v, p)
    }
}
