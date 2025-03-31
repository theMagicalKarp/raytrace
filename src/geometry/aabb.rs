use crate::geometry::axis::Axis;
use crate::interval::Interval;
use crate::ray::Ray;
use itertools::iproduct;
use nalgebra::Vector3;
use std::ops::Add;

#[derive(Default, Debug, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut value = Aabb { x, y, z };
        value.pad_to_minimums();
        value
    }

    pub fn from_points(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        let x = match a.x <= b.x {
            true => Interval::new(a.x, b.x),
            false => Interval::new(b.x, a.x),
        };
        let y = match a.y <= b.y {
            true => Interval::new(a.y, b.y),
            false => Interval::new(b.y, a.y),
        };
        let z = match a.z <= b.z {
            true => Interval::new(a.z, b.z),
            false => Interval::new(b.z, a.z),
        };
        Aabb::new(x, y, z)
    }

    pub fn from_boxes(a: &Aabb, b: &Aabb) -> Self {
        let x = Interval::combine(a.x, b.x);
        let y = Interval::combine(a.y, b.y);
        let z = Interval::combine(a.z, b.z);
        Aabb::new(x, y, z)
    }

    pub fn axis_interval(&self, axis: &Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() && self.x.size() > self.z.size() {
            return Axis::X;
        }
        if self.y.size() > self.x.size() && self.y.size() > self.z.size() {
            return Axis::Y;
        }

        Axis::Z
    }

    pub fn hit(&self, r: &Ray, interval: &Interval) -> bool {
        let ray_origin = r.origin;
        let ray_direction = r.direction;

        let mut t_min = interval.min;
        let mut t_max = interval.max;

        for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
            let axis_interval = self.axis_interval(axis);
            let axis_index = axis.as_index();
            let adinv = 1.0 / ray_direction[axis_index];
            let t0 = (axis_interval.min - ray_origin[axis_index]) * adinv;
            let t1 = (axis_interval.max - ray_origin[axis_index]) * adinv;

            let (t0, t1) = match t0 < t1 {
                true => (t0, t1),
                false => (t1, t0),
            };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vector3<f64>> {
        iproduct!(
            [self.x.max, self.x.min].into_iter(),
            [self.y.max, self.y.min].into_iter(),
            [self.z.max, self.z.min].into_iter()
        )
        .map(|(x, y, z)| Vector3::new(x, y, z))
    }
}

impl Add<Vector3<f64>> for Aabb {
    type Output = Self;
    fn add(self, offset: Vector3<f64>) -> Self::Output {
        Aabb::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}
