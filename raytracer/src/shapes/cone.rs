use crate::{
    intersect::intersection::IntersectionsFactor,
    point::Point,
    ray::Ray,
    util::{equal, solve_quadratic_equation, EPSILON, INFINITY},
    vector::Vector,
};

use super::ShapeLocal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Cone {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }
}

impl Cone {
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Self {
            minimum,
            maximum,
            closed: true,
        }
    }

    fn radius(&self, y: f64) -> f64 {
        y.abs()
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let ray_position = ray.position(t);
        let x = ray_position.x();
        let y = ray_position.y();
        let z = ray_position.z();
        let r = self.radius(y);
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

impl ShapeLocal for Cone {
    fn local_normal_at(&self, point: &Point) -> Vector {
        let (x, y, z) = (point.x(), point.y(), point.z());
        let r = self.radius(y);

        let dist = x * x + z * z;
        if dist <= r * r && y >= self.maximum - EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist <= r * r && y <= self.minimum + EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            let mut ty = (x * x + z * z).sqrt();
            if y > 0.0 {
                ty = -ty;
            };
            Vector::new(x, ty, z)
        }
    }

    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        let dx = local_ray.direction().x();
        let dy = local_ray.direction().y();
        let dz = local_ray.direction().z();
        let ox = local_ray.origin().x();
        let oy = local_ray.origin().y();
        let oz = local_ray.origin().z();

        let a = dx * dx + dz * dz - dy * dy;
        let b = 2.0 * (ox * dx + oz * dz - oy * dy);
        let c = ox * ox + oz * oz - oy * oy;

        let mut roots = IntersectionsFactor::new();
        let cap_roots = self.intersect_cap(local_ray);
        let normal_roots = match equal(a, 0.0) && !equal(b, 0.0) {
            true => vec![-c / 2.0 / b],
            false => solve_quadratic_equation(a, b, c),
        };

        roots.extend(cap_roots.into_iter());
        roots.extend(normal_roots.into_iter().filter(|&t| {
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

    fn test_intersect(
        shape: &Cone,
        origin: Point,
        direction: Vector,
        nroots: usize,
        t0: Option<f64>,
        t1: Option<f64>,
    ) {
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        let xs = shape.local_intersection(&r);
        assert_eq!(xs.len(), nroots);
        if let Some(t0) = t0 {
            assert_float_eq!(xs[0], t0);
        }
        if let Some(t1) = t1 {
            assert_float_eq!(xs[1], t1);
        }
    }

    #[test]
    fn intersecting_cone_with_ray() {
        let shape = Cone::default();

        test_intersect(
            &shape,
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            1,
            Some(5.0),
            None,
        );
        test_intersect(
            &shape,
            Point::new(0.0, 0.0, -5.0),
            Vector::new(1.0, 1.0, 1.0),
            1,
            Some(8.66025),
            None,
        );
        test_intersect(
            &shape,
            Point::new(1.0, 1.0, -5.0),
            Vector::new(-0.5, -1.0, 1.0),
            2,
            Some(4.55006),
            Some(49.44994),
        );
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_halves() {
        test_intersect(
            &Cone::default(),
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 1.0),
            1,
            Some(0.35355),
            None,
        )
    }

    #[test]
    fn intersecting_cone_end_caps() {
        let shape = Cone::new(-0.5, 0.5);

        test_intersect(
            &shape,
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 1.0, 0.0),
            0,
            None,
            None,
        );
        test_intersect(
            &shape,
            Point::new(0.0, 0.0, -0.25),
            Vector::new(0.0, 1.0, 1.0),
            2,
            None,
            None,
        );
        test_intersect(
            &shape,
            Point::new(0.0, 0.0, -0.25),
            Vector::new(0.0, 1.0, 0.0),
            4,
            None,
            None,
        );
    }

    #[test]
    fn computing_normal_vector_on_cone() {
        let shape = Cone::default();
        let test_local_normal = |point: Point, expected_normal: Vector| {
            let n = shape.local_normal_at(&point);
            assert_eq!(n, expected_normal);
        };

        test_local_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0));
        test_local_normal(
            Point::new(1.0, 1.0, 1.0),
            Vector::new(1.0, -f64::sqrt(2.0), 1.0),
        );
        test_local_normal(Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0));
    }
}
