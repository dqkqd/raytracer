use crate::{Color, Point};

// TestPattern used only for testing
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct DummyPattern {}

impl DummyPattern {
    pub fn pattern_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}
