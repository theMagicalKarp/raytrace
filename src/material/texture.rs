use crate::interval::Interval;
use crate::noise::Perlin;
use image::RgbImage;
use nalgebra::Vector3;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Sample {
    fn sample(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32>;
}

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(SolidColor),
    Checkered(Checkered),
    Image(Image),
    Noise(Noise),
}

impl Sample for Texture {
    fn sample(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        match self {
            Texture::SolidColor(texture) => texture.sample(u, v, p),
            Texture::Checkered(texture) => texture.sample(u, v, p),
            Texture::Image(texture) => texture.sample(u, v, p),
            Texture::Noise(texture) => texture.sample(u, v, p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolidColor {
    pub albedo: Vector3<f32>,
}

impl SolidColor {
    pub fn texture(albedo: Vector3<f32>) -> Texture {
        Texture::SolidColor(SolidColor { albedo })
    }
}

impl Sample for SolidColor {
    fn sample(&self, _: f32, _: f32, _: Vector3<f32>) -> Vector3<f32> {
        self.albedo
    }
}

#[derive(Debug, Clone)]
pub struct Checkered {
    pub even: Vector3<f32>,
    pub odd: Vector3<f32>,
    pub scale: f32,
}

impl Checkered {
    pub fn texture(scale: f32, even: Vector3<f32>, odd: Vector3<f32>) -> Texture {
        let scale = 1.0 / scale;
        Texture::Checkered(Checkered { scale, even, odd })
    }
}

impl Sample for Checkered {
    fn sample(&self, _: f32, _: f32, p: Vector3<f32>) -> Vector3<f32> {
        let x_int = (p.x * self.scale).floor() as i32;
        let y_int = (p.y * self.scale).floor() as i32;
        let z_int = (p.z * self.scale).floor() as i32;

        match (x_int + y_int + z_int) % 2 {
            0 => self.even,
            _ => self.odd,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub data: Arc<RgbImage>,
}

impl Image {
    pub fn texture(data: RgbImage) -> Texture {
        let data = Arc::new(data);
        Texture::Image(Image { data })
    }
}

impl Sample for Image {
    fn sample(&self, u: f32, v: f32, _: Vector3<f32>) -> Vector3<f32> {
        let interval = Interval::new(0.0, 1.0);

        let u = interval.clamp(u);
        let v = interval.clamp(v);

        let i = (u * self.data.width() as f32) as u32;
        let j = (v * self.data.height() as f32) as u32;

        let pixel = self.data.get_pixel(i, j);
        let color_scale = 1.0 / 256.0;

        Vector3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) * color_scale
    }
}

#[derive(Debug, Clone)]
pub struct Noise {
    pub perlin: Perlin,
    pub scale: f32,
    pub turbulance: u32,
}

impl Noise {
    pub fn texture(scale: f32, turbulance: u32) -> Texture {
        let perlin = Perlin::default();
        Texture::Noise(Noise {
            perlin,
            scale,
            turbulance,
        })
    }
}

impl Sample for Noise {
    fn sample(&self, _: f32, _: f32, p: Vector3<f32>) -> Vector3<f32> {
        Vector3::from_element(1.0f32) * (self.perlin.turb(p * self.scale, self.turbulance))
    }
}
