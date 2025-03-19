use crate::math::Vector3;
use crate::object::HitRecord;
use crate::ray::Ray;
use rand::rngs::ThreadRng;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
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
        rng: &mut ThreadRng,
    ) -> bool {
        let mut scatter_direction = record.normal + Vector3::random_normal(rng);
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        scattered.origin = record.point;
        scattered.direction = scatter_direction;
        attenuation.e = self.albedo.e;
        true
    }
}
