use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Translate {
    geometry: Box<Geometry>,
    offset: Vector3<f64>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(geometry: Geometry, offset: Vector3<f64>) -> Self {
        let geometry = Box::new(geometry);
        let bbox = geometry.bounding_box() + offset;
        Translate {
            geometry,
            offset,
            bbox,
        }
    }
    pub fn geometry(geometry: Geometry, offset: Vector3<f64>) -> Geometry {
        Geometry::Translate(Translate::new(geometry, offset))
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let offset_ray = Ray::new(r.origin - self.offset, r.direction, r.time);

        if !self.geometry.hit(&offset_ray, interval, record, rng) {
            return false;
        }

        record.point += self.offset;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
