use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{transform::Transform, util::equal};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn w(&self) -> f64 {
        0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        let length = 2.0 * self.dot(normal);
        *self - *normal * length
    }

    pub fn transform(self, transformation: Transform) -> Vector {
        transformation * self
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x(), other.x())
            && equal(self.y(), other.y())
            && equal(self.z(), other.z())
            && equal(self.w(), other.w())
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::util::assert_float_eq;

    use super::*;

    #[test]
    fn create_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_float_eq!(v.x(), 1.0);
        assert_float_eq!(v.y(), 2.0);
        assert_float_eq!(v.z(), 3.0);
    }

    #[test]
    fn add_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1 + v2, Vector::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn minus_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vector::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vector_multiply_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn vector_divide_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn vector_magnitude() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));

        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_float_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn vector_normalize() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));

        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = f64::sqrt(14.0);
        assert_eq!(
            v.normalize(),
            Vector::new(1.0 / norm, 2.0 / norm, 3.0 / norm)
        );
    }

    #[test]
    fn normalized_vector_has_unit_magnitude() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_float_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn vector_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_float_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn vector_cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn vector_reflected_through_normal_vector() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(&normal), Vector::new(1.0, 1.0, 0.0),);

        let v = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(
            std::f64::consts::FRAC_1_SQRT_2,
            std::f64::consts::FRAC_1_SQRT_2,
            0.0,
        );
        assert_eq!(v.reflect(&normal), Vector::new(1.0, 0.0, 0.0));
    }
}
