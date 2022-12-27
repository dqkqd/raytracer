use crate::{intersect::IntersectionsFactor, object::ObjectLocal, Point, Ray, Shape, Vector};

use super::shape::ShapeKind;

// use for test shape's behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestShape {}

#[allow(dead_code)]
impl TestShape {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::TestShape(TestShape {}))
    }
}

impl ObjectLocal for TestShape {
    fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x(), point.y(), point.z())
    }

    fn local_intersection(&self, _: &Ray) -> IntersectionsFactor {
        unimplemented!("`TestShape` intersection should not be called")
    }
}
