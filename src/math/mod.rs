use nalgebra::Vector3;
use rand::prelude::*;

pub fn random_vector(rng: &mut ThreadRng) -> Vector3<f64> {
    Vector3::new(
        rng.random_range(-1.0f64..1.0f64),
        rng.random_range(-1.0f64..1.0f64),
        rng.random_range(-1.0f64..1.0f64),
    )
}

pub fn random_normal(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let position = random_vector(rng);
        let lensq = position.norm_squared();
        if f64::EPSILON < lensq && lensq <= 1.0 {
            return position / lensq.sqrt();
        }
    }
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    v.x.abs() < 1e-8 && v.y.abs() < 1e-8 && v.z.abs() < 1e-8
}

pub fn reflect(a: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    a - n * (a.dot(n) * 2.0)
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min((-uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}
