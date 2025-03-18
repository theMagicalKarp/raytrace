use num::traits::real::Real;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T> {
    e: [T; 3],
}

pub trait Number: fmt::Display + Real {}
impl Number for f32 {}
impl Number for f64 {}

impl<T> Vector3<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> T {
        self.e[0]
    }

    pub fn y(&self) -> T {
        self.e[1]
    }

    pub fn z(&self) -> T {
        self.e[2]
    }

    pub fn length(&self) -> T {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, v: Vector3<T>) -> T {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn sum(&self) -> T {
        self.e[0] + self.e[1] + self.e[2]
    }
}

impl<T> Neg for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl<T> Add for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl<T> Add<T> for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self::new(self.e[0] + other, self.e[1] + other, self.e[2] + other)
    }
}

impl<T> Sub for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl<T> Sub<T> for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        Self::new(self.e[0] - other, self.e[1] - other, self.e[2] - other)
    }
}

impl<T> Mul for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl<T> Div for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Number,
{
    type Output = Self;

    fn div(self, t: T) -> Self::Output {
        Self::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl<T> fmt::Display for Vector3<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vector3_mul() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 * v2;
        assert_eq!(v3, Vector3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_vector3_div() {
        let v1 = Vector3::new(4.0, 9.0, 16.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        let v3 = v1 / v2;
        assert_eq!(v3, Vector3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_vector3_normalize() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        let normalized_v = v.normalize();
        assert_eq!(normalized_v, Vector3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_vector3_neg() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        let neg_v = -v;
        assert_eq!(neg_v, Vector3::new(-1.0, 2.0, -3.0));
    }
}

// pub fn dot<T>(u: Vector3<T>, v: Vector3<T>)  -> T where T: Number {
//     u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
// }

// pub fn cross<T>(u: Vector3<T>, v: Vector3<T>) -> Vector3<T> where T: Number {
//     Vector3::new(
//         u.e[1] * v.e[2] - u.e[2] * v.e[1],
//         u.e[2] * v.e[0] - u.e[0] * v.e[2],
//         u.e[0] * v.e[1] - u.e[1] * v.e[0],
//     )
// }

// pub fn unit_vector<T>(v: Vector3<T>) -> Vector3<T>  where T: Number {
//     v / v.length()
// }
