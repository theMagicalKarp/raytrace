use crate::interval::Interval;
use crate::math::Vector3;
use crate::object::HitRecord;
use crate::object::Hittable;
use crate::ray::Ray;
use image::RgbImage;
use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,

    pub center: Vector3<f32>,
    pub pixel00_loc: Vector3<f32>,
    pub pixel_delta_u: Vector3<f32>,
    pub pixel_delta_v: Vector3<f32>,
    pub image: RgbImage,
    pub samples: u32,
    pub samples_scale: f32,
    pub max_bounces: u32,
    pub rng: ThreadRng,
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height: u32 = std::cmp::max(1, (image_width as f32 / aspect_ratio) as u32);

        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
        let center = Vector3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let image = RgbImage::new(image_width, image_height);

        let samples = 100;
        let samples_scale = 1.0 / (samples as f32);
        let rng = rand::rng();

        let max_bounces = 50;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image,
            samples,
            samples_scale,
            rng,
            max_bounces,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                print!(
                    "Rendering: {}%\r",
                    (x + y * self.image_width) * 100 / (self.image_width * self.image_height)
                );
                let mut color = Vector3::default();
                for _ in 0..self.samples {
                    let r = self.get_ray(x, y);
                    color = color + self.ray_color(&r, self.max_bounces, world);
                }

                color = color * self.samples_scale;
                let intensity = Interval::new(0.0, 0.999);

                let r = intensity.clamp(linear_to_gamma(color.x())) * 256.0;
                let g = intensity.clamp(linear_to_gamma(color.y())) * 256.0;
                let b = intensity.clamp(linear_to_gamma(color.z())) * 256.0;

                self.image
                    .put_pixel(x, y, image::Rgb([r as u8, g as u8, b as u8]));
            }
        }
        println!();
    }

    pub fn get_ray(&mut self, x: u32, y: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x() + x as f32))
            + (self.pixel_delta_v * (offset.y() + y as f32));

        Ray::new(self.center, pixel_sample - self.center)
    }

    pub fn sample_square(&mut self) -> Vector3<f32> {
        let x = self.rng.random_range(-0.5..0.5);
        let y = self.rng.random_range(-0.5..0.5);
        Vector3::new(x, y, 0.0)
    }

    pub fn ray_color(&mut self, ray: &Ray, depth: u32, world: &dyn Hittable) -> Vector3<f32> {
        if depth == 0 {
            return Vector3::default();
        }

        let mut hit_record = HitRecord::default();
        let interval = Interval::new(0.001, f32::INFINITY);

        if world.hit(ray, &interval, &mut hit_record) {
            let mut scattered = Ray::default();
            let mut attenuation = Vector3::<f32>::default();

            if hit_record.material.scatter(
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered,
                &mut self.rng,
            ) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Vector3::default();
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vector3::new(1.0 - a, 1.0 - a, 1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
    }
}
