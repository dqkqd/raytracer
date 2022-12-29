use crate::{transform::InversedTransform, util::equal, Color, Point, Transform};

use super::PatternLocal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingPattern {
    left_color: Color,
    right_color: Color,
    inversed_transform: InversedTransform,
}

impl RingPattern {
    pub(crate) fn new(left_color: Color, right_color: Color) -> RingPattern {
        RingPattern {
            left_color,
            right_color,
            inversed_transform: Some(Transform::identity()),
        }
    }
}

impl PatternLocal for RingPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let x = point.x();
        let z = point.z();
        match equal((x * x + z * z).sqrt().floor() % 2.0, 0.0) {
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
    fn ring_should_extend_in_both_x_and_z() {
        let p = RingPattern::new(color::WHITE, color::BLACK);
        assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(p.pattern_at(&Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 1.0)), color::BLACK);
        assert_eq!(p.pattern_at(&Point::new(0.708, 0.0, 0.708)), color::BLACK);
    }
}
