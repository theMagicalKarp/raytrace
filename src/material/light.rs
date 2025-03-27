use crate::material::Material;
use crate::material::texture::Texture;
use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Light {
    pub texture: Arc<dyn Texture>,
}

unsafe impl Sync for Light {}
unsafe impl Send for Light {}

impl Light {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Light { texture }
    }
}

impl Material for Light {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Vector3<f32>, _: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        self.texture.value(u, v, p)
    }
}
