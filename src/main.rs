mod camera;
mod interval;
mod math;
mod object;
mod ray;

use camera::Camera;
use math::Vector3;
use object::HittableList;
use object::Sphere;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let mut camera = Camera::new(aspect_ratio, image_width);
    let mut world = HittableList::new();

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(-0.5, 0.0, -0.5),
        radius: 0.05,
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    world.add(Box::new(Sphere {
        center: Vector3::new(5.0, 0.5, -10.0),
        radius: 2.0,
    }));

    camera.render(&world);
    match camera.image.save("render.png") {
        Ok(_) => println!("Image saved"),
        Err(e) => println!("Error saving image: {}", e),
    }
}
