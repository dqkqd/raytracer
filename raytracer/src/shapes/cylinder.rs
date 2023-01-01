use crate::{
    intersect::intersection::IntersectionsFactor,
    point::Point,
    ray::Ray,
    util::{equal, solve_quadratic_equation, ESPILON, INFINITY},
    vector::Vector,
};

use super::ShapeLocal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }
}

impl Cylinder {
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Self {
            minimum,
            maximum,
            closed: true,
        }
    }

    fn radius(&self) -> f64 {
        1.0
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let ray_position = ray.position(t);
        let x = ray_position.x();
        let z = ray_position.z();
        let r = self.radius();
        (x * x + z * z) <= r * r
    }

    fn intersect_cap(&self, ray: &Ray) -> IntersectionsFactor {
        let mut roots = IntersectionsFactor::new();

        if !self.closed || equal(ray.direction().y(), 0.0) {
            return roots;
        }

        let t = (self.minimum - ray.origin().y()) / ray.direction().y();
        if self.check_cap(ray, t) {
            roots.push(t);
        }

        let t = (self.maximum - ray.origin().y()) / ray.direction().y();
        if self.check_cap(ray, t) {
            roots.push(t);
        }

        roots
    }
}

impl ShapeLocal for Cylinder {
    fn local_normal_at(&self, point: &Point) -> Vector {
        let (x, y, z) = (point.x(), point.y(), point.z());
        let r = self.radius();

        let dist = x * x + z * z;
        if dist <= r * r && y >= self.maximum - ESPILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist <= r * r && y <= self.minimum + ESPILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(x, 0.0, z)
        }
    }

    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        let dx = local_ray.direction().x();
        let dz = local_ray.direction().z();
        let ox = local_ray.origin().x();
        let oz = local_ray.origin().z();

        let a = dx * dx + dz * dz;
        let b = 2.0 * ox * dx + 2.0 * oz * dz;
        let c = ox * ox + oz * oz - 1.0;

        let mut roots = self.intersect_cap(local_ray);

        roots.extend(solve_quadratic_equation(a, b, c).into_iter().filter(|&t| {
            let y = local_ray.position(t).y();
            self.minimum < y && y < self.maximum
        }));
        roots
    }
}

#[cfg(test)]
mod test {

    use crate::util::assert_float_eq;

    use super::*;

    #[test]
    fn ray_misses_cylinder() {
        let cyl = Cylinder::default();
        let test_intersect = |origin: Point, direction: Vector| {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.local_intersection(&r);
            assert_eq!(xs.len(), 0);
        };

        test_intersect(Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test_intersect(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test_intersect(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn ray_strikes_cylinder() {
        let cyl = Cylinder::default();
        let test_intersect = |origin: Point, direction: Vector, nroots: usize, t0: f64, t1: f64| {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.local_intersection(&r);
            assert_eq!(xs.len(), nroots);
            if nroots > 0 {
                assert_float_eq!(xs[0], t0);
            }
            if nroots > 1 {
                assert_float_eq!(xs[1], t1);
            }
        };

        test_intersect(
            Point::new(1.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            1,
            5.0,
            5.0,
        );
        test_intersect(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            2,
            4.0,
            6.0,
        );
        test_intersect(
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.1, 1.0, 1.0),
            2,
            6.80798,
            7.08872,
        );
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let cyl = Cylinder::default();
        let test_local_normal = |p: Point, expected_normal: Vector| {
            let n = cyl.local_normal_at(&p);
            assert_eq!(n, expected_normal);
        };

        test_local_normal(Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        test_local_normal(Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0));
        test_local_normal(Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0));
        test_local_normal(Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn default_minimum_and_maximum_of_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(cyl.minimum, -INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = Cylinder::new(1.0, 2.0);
        assert!(cyl.closed);

        let test_intersect = |origin: Point, direction: Vector, nroots: usize| {
            let r = Ray::new(origin, direction.normalize());
            let xs = cyl.local_intersection(&r);
            assert_eq!(xs.len(), nroots);
        };

        test_intersect(Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2);
        test_intersect(Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2);
        test_intersect(Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2);
        test_intersect(Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2);
        test_intersect(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2);
    }

    #[test]
    fn normal_vector_on_closed_cylinder_end_caps() {
        let cyl = Cylinder::new(1.0, 2.0);
        assert!(cyl.closed);

        let test_local_normal = |p: Point, expected_normal: Vector| {
            let n = cyl.local_normal_at(&p);
            assert_eq!(n, expected_normal);
        };

        test_local_normal(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        test_local_normal(Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        test_local_normal(Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0));
        test_local_normal(Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test_local_normal(Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test_local_normal(Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0));
    }
}
