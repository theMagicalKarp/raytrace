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
