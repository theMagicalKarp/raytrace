use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::material::isotropic::Isotropic;
use crate::material::texture::Texture;
use crate::ray::Ray;
use nalgebra::Vector3;
use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Volume {
    boundry: Box<Geometry>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl Volume {
    pub fn new(boundry: Geometry, density: f64, texture: Texture) -> Self {
        let boundry = Box::new(boundry);
        let neg_inv_density = -1.0 / density;
        let phase_function = Isotropic::material(texture);
        Volume {
            boundry,
            neg_inv_density,
            phase_function,
        }
    }
    pub fn geometry(boundry: Geometry, density: f64, texture: Texture) -> Geometry {
        Geometry::Volume(Volume::new(boundry, density, texture))
    }
}

impl Hittable for Volume {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut record_a = HitRecord::default();
        let mut record_b = HitRecord::default();

        if !self
            .boundry
            .hit(r, &Interval::universe(), &mut record_a, rng)
        {
            return false;
        }

        if !self.boundry.hit(
            r,
            &Interval::new(record_a.t + 0.0001, f64::INFINITY),
            &mut record_b,
            rng,
        ) {
            return false;
        }

        if record_a.t < interval.min {
            record_a.t = interval.min;
        }

        if record_b.t > interval.max {
            record_b.t = interval.max;
        }

        if record_a.t >= record_b.t {
            return false;
        }

        if record_a.t < 0.0 {
            record_a.t = 0.0;
        }

        let ray_length = r.direction.norm();
        let distance_inside_boundary = (record_b.t - record_a.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        record.t = record_a.t + hit_distance / ray_length;
        record.point = r.at(record.t);

        record.normal = Vector3::new(1.0, 0.0, 0.0);
        record.front_face = true;
        record.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundry.bounding_box()
    }
}
