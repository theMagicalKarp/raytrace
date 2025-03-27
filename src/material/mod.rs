pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;
pub mod texture;

use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

pub trait Material: Debug + Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32>;
}
