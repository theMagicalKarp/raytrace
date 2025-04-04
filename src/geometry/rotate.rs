use crate::geometry::Geometry;
use crate::geometry::HitRecord;
use crate::geometry::Hittable;
use crate::geometry::aabb::Aabb;
use crate::geometry::axis::Axis;
use crate::interval::Interval;
use crate::ray::Ray;
use nalgebra::Rotation3;
use nalgebra::Vector3;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct Rotate {
    geometry: Box<Geometry>,
    rotation: Rotation3<f64>,
    bbox: Aabb,
}

impl Rotate {
    pub fn new(geometry: Geometry, axis: Axis, angle: f64) -> Self {
        let rotation: Rotation3<f64> = Rotation3::from_axis_angle(
            &match axis {
                Axis::X => Vector3::x_axis(),
                Axis::Y => Vector3::y_axis(),
                Axis::Z => Vector3::z_axis(),
            },
            angle.to_radians(),
        );

        let rotated_vertices: Vec<Vector3<f64>> = geometry
            .bounding_box()
            .vertices()
            .map(|vertex| rotation * vertex)
            .collect();

        let min = rotated_vertices.iter().fold(
            Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            |culm, vert| culm.inf(vert),
        );
        let max = rotated_vertices.iter().fold(
            Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            |culm, vert| culm.sup(vert),
        );

        let bbox = Aabb::from_points(min, max);
        let geometry = Box::new(geometry);

        Rotate {
            geometry,
            rotation,
            bbox,
        }
    }
    pub fn geometry(geometry: Geometry, axis: Axis, angle: f64) -> Geometry {
        Geometry::Rotate(Rotate::new(geometry, axis, angle))
    }
}

impl Hittable for Rotate {
    fn hit(
        &self,
        r: &Ray,
        interval: &Interval,
        record: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        // Transform the ray from world space to object space.
        let rotated_origin = self.rotation.inverse() * r.origin;
        let rotated_direction = self.rotation.inverse() * r.direction;
        let rotated_ray = Ray::new(rotated_origin, rotated_direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.geometry.hit(&rotated_ray, interval, record, rng) {
            return false;
        }

        // Transform the intersection from object space back to world space.
        record.point = self.rotation * record.point;
        record.normal = self.rotation * record.normal;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
