use crate::math::Vector3;
use crate::object::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool;
}

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
        _: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vector3::random_normal();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        scattered.origin = record.point;
        scattered.direction = scatter_direction;
        attenuation.e = self.albedo.e;
        true
    }
}

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
        let mut reflected = r_in.direction.reflect(record.normal);
        reflected = reflected.normalize() + (Vector3::random_normal() * self.roughness);

        scattered.origin = record.point;
        scattered.direction = reflected;
        attenuation.e = self.albedo.e;
        scattered.direction.dot(record.normal) > 0.0
    }
}

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
        attenuation.e = [1.0, 1.0, 1.0];
        let r_index = match record.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let normalized_direction = r_in.direction.normalize();
        let refraced = normalized_direction.refract(record.normal, r_index);
        scattered.origin = record.point;
        scattered.direction = refraced;

        true
    }
}
