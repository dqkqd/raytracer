use crate::{intersect::IntersectionsFactor, Point, Ray, Vector};

use super::ShapeLocal;

// use for test shape's behavior
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Dummy {}

impl ShapeLocal for Dummy {
    fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x(), point.y(), point.z())
    }

    fn local_intersection(&self, _: &Ray) -> IntersectionsFactor {
        vec![1.0]
    }
}
