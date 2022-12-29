use crate::{transform::InversedTransform, Color, Point, Transform};

use super::PatternLocal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientPattern {
    left_color: Color,
    right_color: Color,
    inversed_transform: InversedTransform,
}

impl GradientPattern {
    pub(crate) fn new(left_color: Color, right_color: Color) -> GradientPattern {
        GradientPattern {
            left_color,
            right_color,
            inversed_transform: Some(Transform::identity()),
        }
    }
}

impl PatternLocal for GradientPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let distance = self.right_color - self.left_color;
        let fraction = point.x() - point.x().floor();
        self.left_color + distance * fraction
    }
}

#[cfg(test)]
mod test {
    use crate::color;

    use super::*;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let p = GradientPattern::new(color::WHITE, color::BLACK);
        assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            p.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            p.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            p.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
