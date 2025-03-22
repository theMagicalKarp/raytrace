use crate::config::CameraOptions;
use crate::interval::Interval;
use crate::math;
use crate::object::HitRecord;
use crate::object::Hittable;
use crate::ray::Ray;
use image::RgbImage;
use nalgebra::Vector3;
use rand::prelude::*;
use std::f32::consts::PI;
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

    pub defocus_disk_u: Vector3<f32>,
    pub defocus_disk_v: Vector3<f32>,
    pub defocus_angle: f32,
}

fn random_in_unit_disk() -> Vector3<f32> {
    let mut rng = rand::rng();

    loop {
        let p = Vector3::new(
            rng.random_range(-1.0f32..1.0f32),
            rng.random_range(-1.0f32..1.0f32),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn degrees_to_radians(degress: f32) -> f32 {
    degress * PI / 180.0
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
            return attenuation.component_mul(&ray_color(&scattered, depth - 1, world));
        }

        return Vector3::default();
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);

    Vector3::new(1.0 - a, 1.0 - a, 1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let (image_width, image_height) = options.get_dimensions();

        let look_from = Vector3::from_iterator(options.look_from.iter().copied());
        let look_at = Vector3::from_iterator(options.look_at.iter().copied());
        let center = Vector3::from_iterator(options.look_from.iter().copied());
        let vup = Vector3::from_iterator(options.vup.iter().copied());

        let fov = options.fov;
        let theta = degrees_to_radians(fov);

        let defocus_angle = options.defocus_angle;
        let focus_dist = options.focus_dist;

        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * focus_dist;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-v);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - (focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
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

        drop(tx);
        let mut image = RgbImage::new(self.image_width, self.image_height);
        let total = self.image_width * self.image_height;
        let print_at = (total as f32 * 0.05) as usize;
        let is_tty = atty::is(atty::Stream::Stdout);
        for (i, (x, y, pixel)) in rx.iter().enumerate() {
            let percent = (x + y * self.image_width) * 100 / (total);
            let msg = format!("Rendering: {:3}% ({}/{})", percent, i, total);
            if is_tty {
                print!("{}\r", msg);
            } else if i % print_at == 0 {
                println!("{}", msg);
            }
            image.put_pixel(x, y, pixel);
        }
        println!("Rendering: 100% ({}/{})", total, total);
        image
    }

    pub fn get_pixel(&self, world: Arc<dyn Hittable>, x: u32, y: u32) -> image::Rgb<u8> {
        let mut color = Vector3::default();
        for _ in 0..self.samples {
            let r = self.get_ray(x, y);
            color += ray_color(&r, self.max_bounces, world.clone());
        }

        color *= self.samples_scale;
        let intensity = Interval::new(0.0, 0.999);

        let r = intensity.clamp(linear_to_gamma(color.x)) * 256.0;
        let g = intensity.clamp(linear_to_gamma(color.y)) * 256.0;
        let b = intensity.clamp(linear_to_gamma(color.z)) * 256.0;

        image::Rgb([r as u8, g as u8, b as u8])
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = math::random_box(-0.5f32..0.5f32);
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + x as f32))
            + (self.pixel_delta_v * (offset.y + y as f32));

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn defocus_disk_sample(&self) -> Vector3<f32> {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
