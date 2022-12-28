use crate::{intersect::IntersectionsFactor, Point, Ray, Shape, Vector};

use super::{shape::ShapeKind, ShapeLocal};

// use for test shape's behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestShape {}

#[allow(dead_code)]
impl TestShape {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::TestShape(TestShape {}))
    }
}

impl ShapeLocal for TestShape {
    fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x(), point.y(), point.z())
    }

    fn local_intersection(&self, _: &Ray) -> IntersectionsFactor {
        vec![1.0]
    }
}
