use crate::{util::solve_quadratic_equation, IntersectionsFactor, Point, Ray, Vector};

use super::ShapeLocal;

// Sphere always located at (0,0,0) with radius 1.0
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Sphere {}

impl Sphere {
    fn origin(&self) -> Point {
        Point::default()
    }
}

impl ShapeLocal for Sphere {
    fn local_normal_at(&self, point: &Point) -> Vector {
        (*point - self.origin()).normalize()
    }

    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        let sphere_to_ray = local_ray.origin() - self.origin();
        let a = local_ray.direction().dot(&local_ray.direction());
        let b = 2.0 * local_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        solve_quadratic_equation(a, b, c)
    }
}

#[cfg(test)]
mod test {

    use crate::{Shape, Vector};

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Shape::sphere();
        let intersections = sphere.local_intersection(&ray);
        assert_eq!(intersections, [4.0, 6.0]);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Shape::sphere();
        let intersections = sphere.local_intersection(&ray);
        assert_eq!(intersections, [5.0]);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Shape::sphere();
        let intersections = sphere.local_intersection(&ray);
        assert_eq!(intersections, []);
    }

    #[test]
    fn ray_originate_inside_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Shape::sphere();
        let intersections = sphere.local_intersection(&ray);
        assert_eq!(intersections, [-1.0, 1.0]);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Shape::sphere();
        let intersections = sphere.local_intersection(&ray);
        assert_eq!(intersections, [-6.0, -4.0]);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Shape::sphere();
        let n = s.local_normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Shape::sphere();
        let n = s.local_normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Shape::sphere();
        let n = s.local_normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Shape::sphere();
        let v = f64::sqrt(3.0) / 3.0;
        let n = s.local_normal_at(&Point::new(v, v, v));
        assert_eq!(n, Vector::new(v, v, v));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Shape::sphere();
        let v = f64::sqrt(3.0);
        let n = s.local_normal_at(&Point::new(v, v, v));
        assert_eq!(n, n.normalize());
    }
}
