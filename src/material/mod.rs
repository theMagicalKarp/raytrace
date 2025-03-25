use crate::math;
use crate::object::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::prelude::*;
use std::fmt::Debug;

pub fn near_zero(v: &Vector3<f32>) -> bool {
    v.x.abs() < f32::EPSILON && v.y.abs() < f32::EPSILON && v.z.abs() < f32::EPSILON
}

pub fn reflect(a: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    a - n * (a.dot(n) * 2.0)
}

pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min((-uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}

pub trait Material: Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Lambertian { albedo }
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
        attenuation.copy_from(&self.albedo);
        true
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, roughness: f32) -> Self {
        Metal { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
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
}

#[derive(Debug)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.copy_from(&Vector3::new(1.0, 1.0, 1.0));
        let r_index = match record.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let normalized_direction = r_in.direction.normalize();

        let cos_theta = f32::min((-normalized_direction).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = r_index * sin_theta > 1.0;

        scattered.origin = record.point;
        scattered.time = r_in.time;
        let mut rng = rand::rng();
        let random = rng.random_range(0.0f32..1.0f32);

        scattered.direction = match cannot_refract || (reflectance(cos_theta, r_index) > random) {
            true => reflect(&normalized_direction, &record.normal),
            false => refract(&normalized_direction, &record.normal, r_index),
        };

        true
    }
}
