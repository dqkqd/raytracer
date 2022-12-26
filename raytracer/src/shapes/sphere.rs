use crate::{
    intersection::IntersectFactor, transform::transformable, util::solve_quadratic_equation, Point,
    Ray, Transform,
};

use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: f64,
    inversed_transform: Option<Transform>,
}

transformable!(Sphere);

impl Sphere {
    pub fn shape() -> Shape {
        let origin = Point::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Shape::Sphere(Sphere {
            origin,
            radius,
            inversed_transform: Transform::identity().inverse(),
        })
    }

    pub fn intersect_factor(&self, ray: &Ray) -> IntersectFactor {
        let sphere_to_ray = ray.origin() - self.origin;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        solve_quadratic_equation(a, b, c)
    }
}

#[cfg(test)]
mod test {
    use crate::Vector;

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), 4.0);
        assert_eq!(intersections.get(1).unwrap().t(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 1);
        assert_eq!(intersections.get(0).unwrap().t(), 5.0);
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
        assert_eq!(intersections.get(0).unwrap().t(), -1.0);
        assert_eq!(intersections.get(1).unwrap().t(), 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), -6.0);
        assert_eq!(intersections.get(1).unwrap().t(), -4.0);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::shape();
        assert_eq!(s.inversed_transform(), Transform::identity().inverse());
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Transform::translation(2.0, 3.0, 4.0);
        let s = Sphere::shape().with_transform(t);
        assert_eq!(s.inversed_transform(), t.inverse());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape().with_transform(Transform::scaling(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 0);
    }
}
