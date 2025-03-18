use crate::interval::Interval;
use crate::math::Vector3;
use crate::object::HitRecord;
use crate::object::Hittable;
use crate::ray::Ray;

use image::RgbImage;

#[derive(Debug, Clone)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,

    pub center: Vector3<f32>,
    pub pixel00_loc: Vector3<f32>,
    pub pixel_delta_u: Vector3<f32>,
    pub pixel_delta_v: Vector3<f32>,
    pub image: RgbImage,
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

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * x as f32)
                    + (self.pixel_delta_v * y as f32);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&r, world) * 255.99;

                self.image.put_pixel(
                    x,
                    y,
                    image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]),
                );
            }
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Vector3<f32> {
        let mut hit_record = HitRecord::default();
        let interval = Interval::new(0.0, f32::INFINITY);

        if world.hit(ray, &interval, &mut hit_record) {
            return Vector3::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            ) * 0.5;
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vector3::new(1.0 - a, 1.0 - a, 1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
    }
}
