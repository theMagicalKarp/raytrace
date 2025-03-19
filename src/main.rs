mod camera;
mod interval;
mod material;
mod math;
mod object;
mod ray;

use camera::Camera;
use math::Vector3;
use object::HittableList;
use object::Sphere;
use std::rc::Rc;

use crate::material::Lambertian;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let mut camera = Camera::new(aspect_ratio, image_width);
    let mut world = HittableList::new();

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5))),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(-0.5, 0.0, -0.5),
        radius: 0.05,
        material: Rc::new(Lambertian::new(Vector3::new(1.0, 0.75, 0.79))),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(5.0, 0.5, -10.0),
        radius: 2.0,
        material: Rc::new(Lambertian::new(Vector3::new(1.0, 0.75, 0.79))),
    }));

    camera.render(&world);
    match camera.image.save("render.png") {
        Ok(_) => println!("Image saved"),
        Err(e) => println!("Error saving image: {}", e),
    }
}
