use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, time: f64) -> Vector3<f64> {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction, 0.0);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_ray_at() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction, 0.0);
        let time = 2.0;
        let expected_position = Vector3::new(9.0, 12.0, 15.0);
        assert_eq!(ray.at(time), expected_position);
    }

    #[test]
    fn test_ray_at_zero_time() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction, 0.0);
        let time = 0.0;
        assert_eq!(ray.at(time), origin);
    }

    #[test]
    fn test_ray_at_negative_time() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction, 0.0);
        let time = -1.0;
        let expected_position = Vector3::new(-3.0, -3.0, -3.0);
        assert_eq!(ray.at(time), expected_position);
    }
}
