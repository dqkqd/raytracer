use crate::{intersect::IntersectionsFactor, util::solve_linear_equation, Point, Vector};

use super::{
    shape::{Shape, ShapeKind},
    ShapeLocal,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::Plane(Plane {}))
    }
}
impl ShapeLocal for Plane {
    fn local_normal_at(&self, _: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
    fn local_intersection(&self, local_ray: &crate::Ray) -> IntersectionsFactor {
        let a = local_ray.direction().y();
        let b = local_ray.origin().y();
        solve_linear_equation(a, b)
    }
}

#[cfg(test)]
mod test {
    use crate::Ray;

    use super::*;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::shape();
        let n1 = p.local_normal_at(&Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_plane() {
        let p = Plane::shape();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersection(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::shape();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersection(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::shape();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.local_intersection(&r);
        assert_eq!(xs, [1.0]);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::shape();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.local_intersection(&r);
        assert_eq!(xs, [1.0]);
    }
}
