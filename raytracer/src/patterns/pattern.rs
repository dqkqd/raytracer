use crate::{
    color::Color,
    point::Point,
    shapes::shape::Shape,
    transform::{transformable, InversedTransform, Transform, Transformable},
};

use super::{
    dummy::DummyPattern, CheckerPattern, GradientPattern, PatternKind, PatternLocal, PatternWorld,
    RingPattern, StripedPattern,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pattern {
    pattern: PatternKind,
    inversed_transform: InversedTransform,
    transpose_inversed_transform: InversedTransform,
}

transformable!(Pattern);

#[allow(dead_code)]
impl Pattern {
    fn new(pattern: PatternKind) -> Pattern {
        Pattern {
            pattern,
            inversed_transform: Some(Transform::identity()),
            transpose_inversed_transform: Some(Transform::identity()),
        }
    }

    pub fn stripe(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::Stripe(StripedPattern::new(
            left_color,
            right_color,
        )))
    }

    pub fn ring(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::Ring(RingPattern::new(left_color, right_color)))
    }

    pub fn gradient(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::Gradient(GradientPattern::new(
            left_color,
            right_color,
        )))
    }

    pub fn checker(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::Checker(CheckerPattern::new(
            left_color,
            right_color,
        )))
    }

    pub fn dummy() -> Pattern {
        Pattern::new(PatternKind::Dummy(DummyPattern::default()))
    }
}

impl PatternLocal for Pattern {
    fn pattern_at(&self, point: &Point) -> Color {
        match self.pattern {
            PatternKind::Stripe(p) => p.pattern_at(point),
            PatternKind::Dummy(p) => p.pattern_at(point),
            PatternKind::Gradient(p) => p.pattern_at(point),
            PatternKind::Ring(p) => p.pattern_at(point),
            PatternKind::Checker(p) => p.pattern_at(point),
        }
    }
}

impl PatternWorld for Pattern {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color {
        let color_at = || {
            let object_point = world_point.transform(object.inversed_transform()?);
            let pattern_point = object_point.transform(self.inversed_transform()?);
            Some(self.pattern_at(&pattern_point))
        };
        color_at().unwrap_or_default()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn default_pattern_transformation() {
        let p = Pattern::dummy();
        assert_eq!(p.inversed_transform, Some(Transform::identity()))
    }

    #[test]
    fn assigning_transformtion() {
        let m = Transform::scaling(1.0, 2.0, 3.0);
        let p = Pattern::dummy().with_transform(m);
        assert_eq!(p.inversed_transform, m.inverse());
    }

    #[test]
    fn pattern_with_an_object_transformation() {
        let s = Shape::sphere().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let p = Pattern::dummy();
        let c = p.pattern_at_shape(&s, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_a_pattern_transformation() {
        let s = Shape::sphere();
        let p = Pattern::dummy().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let c = p.pattern_at_shape(&s, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn stripe_with_both_an_object_and_a_pattern_transformation() {
        let s = Shape::sphere().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let p = Pattern::dummy().with_transform(Transform::translation(0.5, 1.0, 1.5));
        let c = p.pattern_at_shape(&s, &Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
