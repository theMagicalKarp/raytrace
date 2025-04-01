pub mod aabb;
pub mod axis;
pub mod bvh;
pub mod cube;
pub mod empty;
pub mod quad;
pub mod rotate;
pub mod sphere;
pub mod translate;
pub mod volume;

use crate::geometry::aabb::Aabb;
use crate::geometry::bvh::BvhNode;
use crate::geometry::cube::Cube;
use crate::geometry::empty::Empty;
use crate::geometry::quad::Quad;
use crate::geometry::rotate::Rotate;
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::Translate;
use crate::geometry::volume::Volume;
use crate::interval::Interval;
use crate::material::Material;
use crate::material::lambertian::Lambertian;
use crate::material::texture::SolidColor;
use crate::ray::Ray;
use nalgebra::Vector3;

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Empty(Empty),
    Quad(Quad),
    Sphere(Sphere),
    BvhNode(BvhNode),
    Cube(Cube),
    Translate(Translate),
    Rotate(Rotate),
    Volume(Volume),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        match self {
            Geometry::Empty(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Quad(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Sphere(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::BvhNode(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Cube(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Translate(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Rotate(geomtry) => geomtry.hit(ray, interval, record),
            Geometry::Volume(geomtry) => geomtry.hit(ray, interval, record),
        }
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            Geometry::Empty(geomtry) => geomtry.bounding_box(),
            Geometry::Quad(geomtry) => geomtry.bounding_box(),
            Geometry::Sphere(geomtry) => geomtry.bounding_box(),
            Geometry::BvhNode(geomtry) => geomtry.bounding_box(),
            Geometry::Cube(geomtry) => geomtry.bounding_box(),
            Geometry::Translate(geomtry) => geomtry.bounding_box(),
            Geometry::Rotate(geomtry) => geomtry.bounding_box(),
            Geometry::Volume(geomtry) => geomtry.bounding_box(),
        }
    }
}

pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn default() -> Self {
        HitRecord {
            point: Vector3::default(),
            normal: Vector3::default(),
            t: 0.0,
            front_face: false,
            material: Lambertian::material(SolidColor::texture(Vector3::new(0.98, 0.75, 0.79))),
            u: 0.0,
            v: 0.0,
        }
    }
}
