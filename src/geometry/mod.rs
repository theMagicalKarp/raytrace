pub mod aabb;
pub mod axis;
pub mod bvh;
pub mod cube;
pub mod empty;
pub mod quad;
pub mod rotate;
pub mod scale;
pub mod sphere;
pub mod translate;
pub mod triangle;
pub mod volume;
pub mod wavefront;

use crate::geometry::aabb::Aabb;
use crate::geometry::bvh::BvhNode;
use crate::geometry::cube::Cube;
use crate::geometry::empty::Empty;
use crate::geometry::quad::Quad;
use crate::geometry::rotate::Rotate;
use crate::geometry::scale::Scale;
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::Translate;
use crate::geometry::triangle::Triangle;
use crate::geometry::volume::Volume;
use crate::geometry::wavefront::Wavefront;
use crate::interval::Interval;
use crate::material::Material;
use crate::material::lambertian::Lambertian;
use crate::material::texture::SolidColor;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;

pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool;
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
    Triangle(Triangle),
    Wavefront(Wavefront),
    Scale(Scale),
}

impl Hittable for Geometry {
    fn hit(
        &self,
        ray: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        match self {
            Geometry::Empty(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Quad(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Sphere(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::BvhNode(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Cube(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Translate(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Rotate(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Volume(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Triangle(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Wavefront(geometry) => geometry.hit(ray, interval, record, rng),
            Geometry::Scale(geometry) => geometry.hit(ray, interval, record, rng),
        }
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            Geometry::Empty(geometry) => geometry.bounding_box(),
            Geometry::Quad(geometry) => geometry.bounding_box(),
            Geometry::Sphere(geometry) => geometry.bounding_box(),
            Geometry::BvhNode(geometry) => geometry.bounding_box(),
            Geometry::Cube(geometry) => geometry.bounding_box(),
            Geometry::Translate(geometry) => geometry.bounding_box(),
            Geometry::Rotate(geometry) => geometry.bounding_box(),
            Geometry::Volume(geometry) => geometry.bounding_box(),
            Geometry::Triangle(geometry) => geometry.bounding_box(),
            Geometry::Wavefront(geometry) => geometry.bounding_box(),
            Geometry::Scale(geometry) => geometry.bounding_box(),
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
