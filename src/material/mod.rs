pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

pub trait Material: Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool;
}
