use std::ops::{Add, Sub};

use crate::{util::equal, vector::Vector, Transform};

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        1.0
    }

    pub fn transform(self, transformation: Transform) -> Point {
        transformation * self
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x(), other.x())
            && equal(self.y(), other.y())
            && equal(self.z(), other.z())
            && equal(self.w(), other.w())
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

#[cfg(test)]
mod test {
    use crate::util::assert_float_eq;

    use super::*;

    #[test]
    fn create_new_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_float_eq!(p.x(), 1.0);
        assert_float_eq!(p.y(), 2.0);
        assert_float_eq!(p.z(), 3.0);
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn point_add_vector() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 1.5, 2.0);
        assert_eq!(p + v, Point::new(2.0, 3.5, 5.0));
    }
}
