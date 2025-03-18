use image;

mod math;
mod object;
mod ray;
use crate::object::Hittable;

fn ray_color(ray: ray::Ray, world: &object::HittableList) -> math::Vector3<f32> {
    let mut hit_record = object::HitRecord {
        point: math::Vector3::new(0.0, 0.0, 0.0),
        normal: math::Vector3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.hit(&ray, 0.0, std::f32::INFINITY, &mut hit_record) {
        return math::Vector3::new(
            hit_record.normal.x() + 1.0,
            hit_record.normal.y() + 1.0,
            hit_record.normal.z() + 1.0,
        ) * 0.5;
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    return math::Vector3::new(1.0 - a, 1.0 - a, 1.0 - a) + math::Vector3::new(0.5, 0.7, 1.0) * a;
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = std::cmp::max(1, (image_width as f32 / aspect_ratio) as u32);

    let focal_length = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = math::Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = math::Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = math::Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center
        - math::Vector3::new(0.0, 0.0, focal_length)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut world = object::HittableList::new();
    // world.add(Sphere::new(math::Vector3::new(0.0, 0.0, -1.0), 0.5));

    world.add(Box::new(object::Sphere {
        center: math::Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.add(Box::new(object::Sphere {
        center: math::Vector3::new(-0.5, 0.0, -0.5),
        radius: 0.05,
    }));

    world.add(Box::new(object::Sphere {
        center: math::Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    world.add(Box::new(object::Sphere {
        center: math::Vector3::new(5.0, 0.5, -10.0),
        radius: 2.0,
    }));

    let mut img = image::RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray::new(camera_center, ray_direction);

            let color = ray_color(r, &world) * 255.99;

            img.put_pixel(
                x,
                y,
                image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]),
            );
        }
    }

    img.save("foo.png").unwrap();
}
