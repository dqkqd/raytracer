use crate::Shape;

use super::shape::ShapeKind;

// use for test shape's behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TestShape {}

#[allow(dead_code)]
impl TestShape {
    pub fn shape() -> Shape {
        Shape::new(ShapeKind::TestShape)
    }
}
