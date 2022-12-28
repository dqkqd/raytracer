use crate::{
    transform::{transformable, InversedTransform},
    Color, Point, Shape, Transform, Transformable,
};

use super::{PatternKind, PatternLocal, PatternWorld};

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
}

impl PatternLocal for Pattern {
    fn pattern_at(&self, point: &Point) -> Color {
        match self.pattern {
            PatternKind::StripedPattern(p) => p.pattern_at(point),
            PatternKind::TestPattern(p) => p.pattern_at(point),
            PatternKind::GradientPattern(p) => p.pattern_at(point),
            PatternKind::RingPattern(p) => p.pattern_at(point),
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
    use crate::{patterns::dummy_pattern::TestPattern, Sphere, Transformable};

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

    #[test]
    fn pattern_with_an_object_transformation() {
        let s = Sphere::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let p = TestPattern::pattern();
        let c = p.pattern_at_shape(&s, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_a_pattern_transformation() {
        let s = Sphere::shape();
        let p = TestPattern::pattern().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let c = p.pattern_at_shape(&s, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn stripe_with_both_an_object_and_a_pattern_transformation() {
        let s = Sphere::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let p = TestPattern::pattern().with_transform(Transform::translation(0.5, 1.0, 1.5));
        let c = p.pattern_at_shape(&s, &Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
