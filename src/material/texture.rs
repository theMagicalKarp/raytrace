use crate::interval::Interval;
use crate::noise::Perlin;
use image::Rgb32FImage;
use nalgebra::Vector3;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Sample {
    fn sample(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64>;
}

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(SolidColor),
    Checkered(Checkered),
    Image(Image),
    Noise(Noise),
}

impl Sample for Texture {
    fn sample(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
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
    pub albedo: Vector3<f64>,
}

impl SolidColor {
    pub fn texture(albedo: Vector3<f64>) -> Texture {
        Texture::SolidColor(SolidColor { albedo })
    }
}

impl Sample for SolidColor {
    fn sample(&self, _: f64, _: f64, _: Vector3<f64>) -> Vector3<f64> {
        self.albedo
    }
}

#[derive(Debug, Clone)]
pub struct Checkered {
    pub even: Vector3<f64>,
    pub odd: Vector3<f64>,
    pub scale: f64,
}

impl Checkered {
    pub fn texture(scale: f64, even: Vector3<f64>, odd: Vector3<f64>) -> Texture {
        let scale = 1.0 / scale;
        Texture::Checkered(Checkered { scale, even, odd })
    }
}

impl Sample for Checkered {
    fn sample(&self, _: f64, _: f64, p: Vector3<f64>) -> Vector3<f64> {
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
    pub data: Arc<Rgb32FImage>,
}

impl Image {
    pub fn texture(data: Rgb32FImage) -> Texture {
        let data = Arc::new(data);
        Texture::Image(Image { data })
    }
}

impl Sample for Image {
    fn sample(&self, u: f64, v: f64, _: Vector3<f64>) -> Vector3<f64> {
        let interval = Interval::new(0.0, 1.0);

        let u = interval.clamp(u);
        let v = interval.clamp(v);

        let i = (u * self.data.width() as f64) as u32;
        let j = (v * self.data.height() as f64) as u32;

        let pixel = self.data.get_pixel(i, j);
        Vector3::new(
            pixel[0].powf(2.0) as f64,
            pixel[1].powf(2.0) as f64,
            pixel[2].powf(2.0) as f64,
        ) * 2.0
    }
}

#[derive(Debug, Clone)]
pub struct Noise {
    pub perlin: Perlin,
    pub scale: f64,
    pub turbulance: u32,
}

impl Noise {
    pub fn texture(scale: f64, turbulance: u32) -> Texture {
        let perlin = Perlin::default();
        Texture::Noise(Noise {
            perlin,
            scale,
            turbulance,
        })
    }
}

impl Sample for Noise {
    fn sample(&self, _: f64, _: f64, p: Vector3<f64>) -> Vector3<f64> {
        Vector3::from_element(1.0f64) * (self.perlin.turb(p * self.scale, self.turbulance))
    }
}
