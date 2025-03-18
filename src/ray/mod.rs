use crate::math;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: math::Vector3<f32>,
    pub direction: math::Vector3<f32>,
}

impl Ray {
    pub fn new(origin: math::Vector3<f32>, direction: math::Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, time: f32) -> math::Vector3<f32> {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new() {
        let origin = math::Vector3::new(1.0, 2.0, 3.0);
        let direction = math::Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_ray_at() {
        let origin = math::Vector3::new(1.0, 2.0, 3.0);
        let direction = math::Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        let time = 2.0;
        let expected_position = math::Vector3::new(9.0, 12.0, 15.0);
        assert_eq!(ray.at(time), expected_position);
    }

    #[test]
    fn test_ray_at_zero_time() {
        let origin = math::Vector3::new(1.0, 2.0, 3.0);
        let direction = math::Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        let time = 0.0;
        assert_eq!(ray.at(time), origin);
    }

    #[test]
    fn test_ray_at_negative_time() {
        let origin = math::Vector3::new(1.0, 2.0, 3.0);
        let direction = math::Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        let time = -1.0;
        let expected_position = math::Vector3::new(-3.0, -3.0, -3.0);
        assert_eq!(ray.at(time), expected_position);
    }
}
