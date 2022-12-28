use crate::{color, util::equal, Color, Point};

use super::pattern::{Pattern, PatternKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripedPattern {
    left_color: Color,
    right_color: Color,
}

impl Default for StripedPattern {
    fn default() -> Self {
        StripedPattern {
            left_color: color::WHITE,
            right_color: color::BLACK,
        }
    }
}

impl StripedPattern {
    pub(crate) fn new(left_color: Color, right_color: Color) -> StripedPattern {
        StripedPattern {
            left_color,
            right_color,
        }
    }

    pub fn pattern(left_color: Color, right_color: Color) -> Pattern {
        Pattern::new(PatternKind::StripedPattern(StripedPattern::new(
            left_color,
            right_color,
        )))
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        match equal(point.x().floor() % 2.0, 0.0) {
            true => self.left_color,
            false => self.right_color,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripedPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.left_color, color::WHITE);
        assert_eq!(pattern.right_color, color::BLACK);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripedPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripedPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn stripe_pattern_is_alternates_in_x() {
        let pattern = StripedPattern::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.stripe_at(&Point::new(-1.1, 0.0, 0.0)), color::WHITE);
    }
}
