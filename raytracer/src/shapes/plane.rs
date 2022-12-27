use crate::{Point, Vector};

use super::{
    object::ObjectLocal,
    shape::{Shape, ShapeKind},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::Plane(Plane {}))
    }
}
impl ObjectLocal for Plane {
    fn local_normal_at(&self, _: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
    fn local_intersection(&self, local_ray: &crate::Ray) -> crate::intersect::IntersectionsFactor {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
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
}
