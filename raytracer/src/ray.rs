use crate::{point::Point, transform::Transform, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin() + self.direction() * t
    }

    pub fn transform(self, transformation: Transform) -> Ray {
        transformation * self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0));
        assert_eq!(ray.origin, Point::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, Vector::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn factor_position() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transform::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transform::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
