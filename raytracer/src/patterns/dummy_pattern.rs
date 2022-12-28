use crate::{Color, Point};

use super::{pattern::Pattern, PatternKind};

// TestPattern used only for testing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn pattern() -> Pattern {
        Pattern::new(PatternKind::TestPattern(TestPattern {}))
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}
