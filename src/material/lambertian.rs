use crate::material::Material;
use crate::material::texture::SolidColor;
use crate::material::texture::Texture;
use crate::math;
use crate::math::near_zero;
use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

unsafe impl Sync for Lambertian {}
unsafe impl Send for Lambertian {}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Lambertian { texture }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian::new(Arc::new(SolidColor::default()))
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
        attenuation.copy_from(&self.texture.value(record.u, record.v, record.point));
        true
    }
}
