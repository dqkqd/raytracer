use crate::{
    transform::{transformable, InversedTransform},
    Color, Point, Shape, Transform,
};

use super::{dummy_pattern::TestPattern, stripe::StripedPattern};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
    TestPattern(TestPattern),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternKind,
    inversed_transform: InversedTransform,
}

transformable!(Pattern);

impl Pattern {
    pub fn new(pattern: PatternKind) -> Pattern {
        Pattern {
            pattern,
            inversed_transform: Some(Transform::identity()),
        }
    }

    pub fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color {
        let color_at = || {
            let object_point = world_point.transform(object.inversed_transform()?);
            let pattern_point = object_point.transform(self.inversed_transform()?);
            let c = match self.pattern {
                PatternKind::StripedPattern(s) => s.pattern_at(&pattern_point),
                PatternKind::TestPattern(p) => unimplemented!(),
            };
            Some(c)
        };
        color_at().unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::Transformable;

    use super::*;

    #[test]
    fn default_pattern_transformation() {
        let p = TestPattern::pattern();
        assert_eq!(p.inversed_transform, Some(Transform::identity()))
    }

    #[test]
    fn assigning_transformtion() {
        let m = Transform::scaling(1.0, 2.0, 3.0);
        let p = TestPattern::pattern().with_transform(m);
        assert_eq!(p.inversed_transform, m.inverse());
    }
}
