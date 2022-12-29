use crate::{intersect::IntersectionsFactor, util::check_axis, Point, Vector};

use super::{shape::Shape, ShapeKind, ShapeLocal};

// Cube always located at (0,0,0) and extended from -1 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {}

impl Cube {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::Cube(Cube {}))
    }
}

impl ShapeLocal for Cube {
    fn local_normal_at(&self, _: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersection(&self, local_ray: &crate::Ray) -> IntersectionsFactor {
        let (xtmin, xtmax) = check_axis(local_ray.origin().x(), local_ray.direction().x());
        let (ytmin, ytmax) = check_axis(local_ray.origin().y(), local_ray.direction().y());
        let (ztmin, ztmax) = check_axis(local_ray.origin().z(), local_ray.direction().z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        vec![tmin, tmax]
    }
}

#[cfg(test)]
mod test {

    use crate::{util::assert_float_eq, Ray};

    use super::*;

    #[test]
    fn ray_intersect_a_cube() {
        let c = Cube::shape();
        let test_intersect = |origin: Point, direction: Vector, t1, t2| {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersection(&r);
            assert_eq!(xs.len(), 2);
            assert_float_eq!(xs[0], t1);
            assert_float_eq!(xs[1], t2);
        };

        test_intersect(
            Point::new(5.0, 0.5, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(-5.0, 0.5, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.5, 5.0, 0.0),
            Vector::new(0.0, -1.0, 0.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.5, -5.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.5, 0.0, 5.0),
            Vector::new(0.0, 0.0, -1.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.0, 0.5, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            -1.0,
            1.0,
        );
    }
}
