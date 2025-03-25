use nalgebra::Vector3;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Texture: Debug {
    fn value(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32>;
}

#[derive(Debug)]
pub struct SolidColor {
    pub albedo: Vector3<f32>,
}

impl SolidColor {
    pub fn new(albedo: Vector3<f32>) -> Self {
        SolidColor { albedo }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        SolidColor::new(Vector3::new(1.0, 0.75, 0.79))
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: Vector3<f32>) -> Vector3<f32> {
        self.albedo
    }
}

#[derive(Debug)]
pub struct Checkered {
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
    pub scale: f32,
}

unsafe impl Sync for Checkered {}
unsafe impl Send for Checkered {}

impl Checkered {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        let scale = 1.0 / scale;
        Checkered { scale, even, odd }
    }
}

impl Default for Checkered {
    fn default() -> Self {
        let black = SolidColor::new(Vector3::new(0.01, 0.01, 0.01));
        let white = SolidColor::new(Vector3::new(0.99, 0.99, 0.99));
        Checkered::new(1.0, Arc::new(black), Arc::new(white))
    }
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        let x_int = (p.x * self.scale).floor() as i32;
        let y_int = (p.y * self.scale).floor() as i32;
        let z_int = (p.z * self.scale).floor() as i32;

        match (x_int + y_int + z_int) % 2 {
            0 => self.even.value(u, v, p),
            _ => self.odd.value(u, v, p),
        }
    }
}
