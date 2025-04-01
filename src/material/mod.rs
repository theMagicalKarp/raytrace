pub mod dielectric;
pub mod isotropic;
pub mod lambertian;
pub mod light;
pub mod metal;
pub mod texture;

use crate::geometry::HitRecord;
use crate::material::dielectric::Dielectric;
use crate::material::isotropic::Isotropic;
use crate::material::lambertian::Lambertian;
use crate::material::light::Light;
use crate::material::metal::Metal;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::fmt::Debug;

pub trait Surface {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Metal(Metal),
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Light(Light),
    Isotropic(Isotropic),
}

impl Surface for Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Metal(material) => material.scatter(ray_in, record, attenuation, scattered),
            Material::Dielectric(material) => {
                material.scatter(ray_in, record, attenuation, scattered)
            }
            Material::Lambertian(material) => {
                material.scatter(ray_in, record, attenuation, scattered)
            }
            Material::Light(material) => material.scatter(ray_in, record, attenuation, scattered),
            Material::Isotropic(material) => {
                material.scatter(ray_in, record, attenuation, scattered)
            }
        }
    }
    fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        match self {
            Material::Metal(material) => material.emitted(u, v, p),
            Material::Dielectric(material) => material.emitted(u, v, p),
            Material::Lambertian(material) => material.emitted(u, v, p),
            Material::Light(material) => material.emitted(u, v, p),
            Material::Isotropic(material) => material.emitted(u, v, p),
        }
    }
}
