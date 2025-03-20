use crate::interval::Interval;
use crate::math::Vector3;
use crate::object::HitRecord;
use crate::object::Hittable;
use crate::ray::Ray;
use image::RgbImage;
use std::sync::Arc;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,

    pub center: Vector3<f32>,
    pub pixel00_loc: Vector3<f32>,
    pub pixel_delta_u: Vector3<f32>,
    pub pixel_delta_v: Vector3<f32>,
    pub samples: u32,
    pub samples_scale: f32,
    pub max_bounces: u32,
    pub threads: usize,
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn ray_color(ray: &Ray, depth: u32, world: Arc<dyn Hittable>) -> Vector3<f32> {
    if depth == 0 {
        return Vector3::default();
    }

    let mut hit_record = HitRecord::default();
    let interval = Interval::new(0.001, f32::INFINITY);

    if world.hit(ray, &interval, &mut hit_record) {
        let mut scattered = Ray::default();
        let mut attenuation = Vector3::<f32>::default();

        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }

        return Vector3::default();
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    Vector3::new(1.0 - a, 1.0 - a, 1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
}

pub struct CameraOptions {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples: u32,
    pub max_bounces: u32,
    pub threads: usize,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let image_width = options.image_width;
        let image_height: u32 =
            std::cmp::max(1, (image_width as f32 / options.aspect_ratio) as u32);

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

        let samples = options.samples;
        let samples_scale = 1.0 / (samples as f32);

        let max_bounces = options.max_bounces;
        let threads = options.threads;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples,
            samples_scale,
            max_bounces,
            threads,
        }
    }

    pub fn render(&self, world: Arc<dyn Hittable>) -> RgbImage {
        let pool = ThreadPool::new(self.threads);
        let (tx, rx) = channel();

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let tx = tx.clone();
                let camera = *self;
                let world = Arc::clone(&world);
                pool.execute(move || {
                    tx.send((x, y, camera.get_pixel(world, x, y)))
                        .expect("Failed to send result");
                });
            }
        }

        let mut image = RgbImage::new(self.image_width, self.image_height);
        drop(tx);
        for (x, y, pixel) in rx.iter() {
            print!(
                "Rendering: {}%\r",
                (x + y * self.image_width) * 100 / (self.image_width * self.image_height)
            );
            image.put_pixel(x, y, pixel);
        }
        print!("Rendering: 100%\r");
        println!();

        image
    }

    pub fn get_pixel(&self, world: Arc<dyn Hittable>, x: u32, y: u32) -> image::Rgb<u8> {
        let mut color = Vector3::default();
        for _ in 0..self.samples {
            let r = self.get_ray(x, y);
            color = color + ray_color(&r, self.max_bounces, world.clone());
        }

        color = color * self.samples_scale;
        let intensity = Interval::new(0.0, 0.999);

        let r = intensity.clamp(linear_to_gamma(color.x())) * 256.0;
        let g = intensity.clamp(linear_to_gamma(color.y())) * 256.0;
        let b = intensity.clamp(linear_to_gamma(color.z())) * 256.0;

        image::Rgb([r as u8, g as u8, b as u8])
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Vector3::random_box(-0.5f32..0.5f32);
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x() + x as f32))
            + (self.pixel_delta_v * (offset.y() + y as f32));

        Ray::new(self.center, pixel_sample - self.center)
    }
}
