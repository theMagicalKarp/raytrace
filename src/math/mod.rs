use nalgebra::Vector3;
use rand::prelude::*;
use std::ops::Range;

pub fn random(range: Range<f32>) -> Vector3<f32> {
    let mut rng = rand::rng();
    Vector3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
    )
}

pub fn random_box(range: Range<f32>) -> Vector3<f32> {
    let mut rng = rand::rng();
    Vector3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        0.0,
    )
}

pub fn random_normal() -> Vector3<f32> {
    loop {
        let position = random(-1.0f32..1.0f32);
        let lensq = position.norm_squared();
        if f32::EPSILON < lensq && lensq <= 1.0 {
            return position / lensq.sqrt();
        }
    }
}

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
