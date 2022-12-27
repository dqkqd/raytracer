use crate::{
    object::ObjectLocal, util::solve_quadratic_equation, IntersectionsFactor, Point, Ray, Vector,
};

use super::shape::{Shape, ShapeKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: f64,
}

impl Sphere {
    pub fn shape() -> Shape {
        let origin = Point::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Shape::new(ShapeKind::Sphere(Sphere { origin, radius }))
    }
}

impl ObjectLocal for Sphere {
    fn local_normal_at(&self, point: &Point) -> Vector {
        (*point - self.origin).normalize()
    }

    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        let sphere_to_ray = local_ray.origin() - self.origin;
        let a = local_ray.direction().dot(&local_ray.direction());
        let b = 2.0 * local_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        solve_quadratic_equation(a, b, c)
    }
}

#[cfg(test)]
mod test {

    use crate::{shapes::object::ObjectWorld, util::assert_float_eq, Vector};

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_float_eq!(intersections.get(0).unwrap().t(), 4.0);
        assert_float_eq!(intersections.get(1).unwrap().t(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 1);
        assert_float_eq!(intersections.get(0).unwrap().t(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 0);
    }

    #[test]
    fn ray_originate_inside_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_float_eq!(intersections.get(0).unwrap().t(), -1.0);
        assert_float_eq!(intersections.get(1).unwrap().t(), 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_float_eq!(intersections.get(0).unwrap().t(), -6.0);
        assert_float_eq!(intersections.get(1).unwrap().t(), -4.0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_eq!(n.unwrap(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_eq!(n.unwrap(), Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_eq!(n.unwrap(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::shape();
        let v = f64::sqrt(3.0) / 3.0;
        let n = s.normal_at(&Point::new(v, v, v));
        assert_eq!(n.unwrap(), Vector::new(v, v, v));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::shape();
        let v = f64::sqrt(3.0);
        let n = s.normal_at(&Point::new(v, v, v)).unwrap();
        assert_eq!(n, n.normalize());
    }
}
