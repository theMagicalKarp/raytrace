mod camera;
mod interval;
mod material;
mod math;
mod object;
mod ray;

use camera::Camera;
use camera::CameraOptions;
use math::Vector3;
use object::HittableList;
use object::Sphere;

use crate::material::Lambertian;
use crate::material::Metal;
use std::sync::Arc;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples: u32 = 100;
    let max_bounces: u32 = 50;
    let threads: usize = 8;

    let camera = Camera::new(CameraOptions {
        aspect_ratio,
        image_width,
        samples,
        max_bounces,
        threads,
    });
    let mut world = HittableList::new();

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5))),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.05)),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.85)),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
    }));

    let image = camera.render(Arc::new(world));

    match image.save("render.png") {
        Ok(_) => println!("Image saved"),
        Err(e) => println!("Error saving image: {}", e),
    }
}
