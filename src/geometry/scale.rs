use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Scale {
    geometry: Box<Geometry>,
    scale: Vector3<f64>,
    bbox: Aabb,
}

impl Scale {
    pub fn new(geometry: Geometry, scale: Vector3<f64>) -> Self {
        let geometry = Box::new(geometry);
        let bbox = geometry.bounding_box() * scale;

        Scale {
            geometry,
            scale,
            bbox,
        }
    }

    pub fn geometry(geometry: Geometry, scale: Vector3<f64>) -> Geometry {
        Geometry::Scale(Scale::new(geometry, scale))
    }
}

impl Hittable for Scale {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let scaled_origin = r.origin.component_div(&self.scale);
        let scaled_direction = r.direction.component_div(&self.scale);
        let scaled_ray = Ray::new(scaled_origin, scaled_direction, r.time);

        if !self.geometry.hit(&scaled_ray, interval, record, rng) {
            return false;
        }

        record.point = record.point.component_mul(&self.scale);
        record.normal = record.normal.component_div(&self.scale).normalize();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
