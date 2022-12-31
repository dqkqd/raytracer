use crate::{
    intersect::intersection::IntersectionsFactor,
    point::Point,
    util::{check_axis, equal},
    vector::Vector,
};

use super::ShapeLocal;

// Cube always located at (0,0,0) and extended from -1 to 1.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Cube {}

impl ShapeLocal for Cube {
    fn local_normal_at(&self, point: &Point) -> Vector {
        let maxc = point.x().abs().max(point.y().abs()).max(point.z().abs());
        if equal(maxc, point.x().abs()) {
            Vector::new(point.x(), 0.0, 0.0)
        } else if equal(maxc, point.y().abs()) {
            Vector::new(0.0, point.y(), 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z())
        }
    }

    fn local_intersection(&self, local_ray: &crate::ray::Ray) -> IntersectionsFactor {
        let (xtmin, xtmax) = check_axis(local_ray.origin().x(), local_ray.direction().x());
        let (ytmin, ytmax) = check_axis(local_ray.origin().y(), local_ray.direction().y());
        let (ztmin, ztmax) = check_axis(local_ray.origin().z(), local_ray.direction().z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else if equal(tmin, tmax) {
            vec![tmin]
        } else {
            vec![tmin, tmax]
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{ray::Ray, shapes::shape::Shape, util::assert_float_eq};

    use super::*;

    #[test]
    fn ray_intersect_a_cube() {
        let c = Shape::cube();
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

    #[test]
    fn ray_misses_cube() {
        let c = Shape::cube();
        let test_intersect = |origin: Point, direction: Vector| {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersection(&r);
            assert_eq!(xs.len(), 0);
        };

        test_intersect(
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.2673, 0.5345, 0.8018),
        );
        test_intersect(
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.8018, 0.2673, 0.5345),
        );
        test_intersect(
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.5345, 0.8018, 0.2673),
        );
        test_intersect(Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0));
        test_intersect(Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0));
        test_intersect(Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let c = Shape::cube();
        let test_normal = |point: Point, expected_normal: Vector| {
            let normal = c.local_normal_at(&point);
            assert_eq!(normal, expected_normal);
        };

        test_normal(Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0));
        test_normal(Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0));
        test_normal(Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0));
        test_normal(Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0));
        test_normal(Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0));
        test_normal(Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0));
        test_normal(Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0));
        test_normal(Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0));
    }
}
