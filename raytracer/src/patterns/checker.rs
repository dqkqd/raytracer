use crate::{transform::InversedTransform, util::equal, Color, Point, Transform};

use super::{pattern::Pattern, PatternKind, PatternLocal};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckerPattern {
    left_color: Color,
    right_color: Color,
    inversed_transform: InversedTransform,
}

impl CheckerPattern {
    pub(crate) fn new(left_color: Color, right_color: Color) -> CheckerPattern {
        CheckerPattern {
            left_color,
            right_color,
            inversed_transform: Some(Transform::identity()),
        }
    }

    pub fn pattern(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::CheckerPattern(CheckerPattern::new(
            left_color,
            right_color,
        )))
    }
}

impl PatternLocal for CheckerPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        match equal(
            (point.x().floor() + point.y().floor() + point.z().floor()) % 2.0,
            0.0,
        ) {
            true => self.left_color,
            false => self.right_color,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color;

    use super::*;

    #[test]
    fn checker_should_repeat_in_x() {
        let pattern = CheckerPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.99, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(1.01, 0.0, 0.0)),
            color::BLACK
        );
    }

    #[test]
    fn checker_should_repeat_in_y() {
        let pattern = CheckerPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.99, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 1.01, 0.0)),
            color::BLACK
        );
    }

    #[test]
    fn checker_should_repeat_in_z() {
        let pattern = CheckerPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.99)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.01)),
            color::BLACK
        );
    }
}
