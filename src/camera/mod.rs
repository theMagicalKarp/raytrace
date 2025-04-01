use crate::config::CameraOptions;
use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::interval::Interval;
use crate::material::Surface;
use crate::math;
use crate::ray::Ray;
use image::RgbImage;
use nalgebra::Vector3;
use rand::prelude::*;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::Instant;
use threadpool::ThreadPool;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,

    pub center: Vector3<f64>,
    pub pixel00_loc: Vector3<f64>,
    pub pixel_delta_u: Vector3<f64>,
    pub pixel_delta_v: Vector3<f64>,
    pub samples: u32,
    pub samples_scale: f64,
    pub max_bounces: u32,
    pub threads: usize,

    pub defocus_disk_u: Vector3<f64>,
    pub defocus_disk_v: Vector3<f64>,
    pub defocus_angle: f64,

    pub background: Vector3<f64>,
}

fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::rng();

    loop {
        let p = Vector3::new(
            rng.random_range(-1.0f64..1.0f64),
            rng.random_range(-1.0f64..1.0f64),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let (image_width, image_height) = options.get_dimensions();

        let background = Vector3::from(options.background);
        let look_from = Vector3::from(options.look_from);
        let look_at = Vector3::from(options.look_at);
        let center = Vector3::from(options.look_from);
        let vup = Vector3::from(options.vup);

        let fov = options.fov;
        let theta = degrees_to_radians(fov);

        let defocus_angle = options.defocus_angle;
        let focus_dist = options.focus_dist;

        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-v);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let samples = options.samples;
        let samples_scale = 1.0 / (samples as f64);

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
            background,
        }
    }

    pub fn render(&self, world: &Geometry) -> RgbImage {
        let now = Instant::now();

        let pool = ThreadPool::new(self.threads);
        let (tx, rx) = channel();
        let world = Arc::new(world.clone());
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let tx = tx.clone();
                let camera = *self;
                let world = Arc::clone(&world);
                pool.execute(move || {
                    tx.send((x, y, camera.get_pixel(&world, x, y)))
                        .expect("Failed to send result");
                });
            }
        }

        drop(tx);
        let mut image = RgbImage::new(self.image_width, self.image_height);
        let total = self.image_width * self.image_height;
        let total_digits = total.to_string().len();
        let is_tty = atty::is(atty::Stream::Stdout);
        let print_at = match is_tty {
            true => (total as f64 * 0.01) as usize,
            false => (total as f64 * 0.05) as usize,
        };
        for (i, (x, y, pixel)) in rx.iter().enumerate() {
            if i % print_at == 0 {
                let percent = (x + y * self.image_width) * 100 / (total);

                let msg = format!(
                    "Rendering: {:3}% ({:total_digits$}/{:total_digits$}) {:>16}",
                    percent,
                    i,
                    total,
                    humantime::format_duration(Duration::from_secs(now.elapsed().as_secs()))
                        .to_string()
                );
                if is_tty {
                    print!("{}\r", msg);
                    io::stdout().flush().expect("Flush to STDOUT failed");
                } else {
                    println!("{}", msg);
                }
            }

            image.put_pixel(x, y, pixel);
        }
        println!(
            "Rendering: 100% ({:total_digits$}/{:total_digits$}) {:>16}",
            total,
            total,
            humantime::format_duration(Duration::from_secs(now.elapsed().as_secs())).to_string()
        );
        image
    }

    pub fn get_pixel(&self, world: &Geometry, x: u32, y: u32) -> image::Rgb<u8> {
        let color = (0..self.samples)
            .map(|_| self.ray_color(&self.get_ray(x, y), self.max_bounces, world))
            .sum::<Vector3<f64>>()
            * self.samples_scale;

        let intensity = Interval::new(0.0, 0.999);
        let r = intensity.clamp(linear_to_gamma(color.x)) * 256.0;
        let g = intensity.clamp(linear_to_gamma(color.y)) * 256.0;
        let b = intensity.clamp(linear_to_gamma(color.z)) * 256.0;

        image::Rgb([r as u8, g as u8, b as u8])
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = math::random_box(-0.5f64..0.5f64);
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + x as f64))
            + (self.pixel_delta_v * (offset.y + y as f64));

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;

        let mut rng = rand::rng();
        let t = rng.random_range(0.0f64..1.0f64);

        Ray::new(ray_origin, ray_direction, t)
    }

    pub fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    pub fn ray_color(&self, ray: &Ray, depth: u32, world: &Geometry) -> Vector3<f64> {
        if depth == 0 {
            return Vector3::default();
        }

        let mut hit_record = HitRecord::default();
        let interval = Interval::new(0.001, f64::INFINITY);

        if !world.hit(ray, &interval, &mut hit_record) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Vector3::<f64>::default();
        let color_from_emission =
            hit_record
                .material
                .emitted(hit_record.u, hit_record.v, hit_record.point);

        if !hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return color_from_emission;
        }

        let color_from_scatter =
            attenuation.component_mul(&self.ray_color(&scattered, depth - 1, world));

        color_from_emission + color_from_scatter
    }
}
