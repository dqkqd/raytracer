use crate::{Color, Point, Shape};

use super::{dummy_pattern::TestPattern, stripe::StripedPattern};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
    TestPattern(TestPattern),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternKind,
}

impl Pattern {
    pub fn new(pattern: PatternKind) -> Pattern {
        Pattern { pattern }
    }

    pub fn color_at(&self, point: &Point) -> Color {
        match self.pattern {
            PatternKind::StripedPattern(s) => s.stripe_at(point),
            PatternKind::TestPattern(_) => unimplemented!(),
        }
    }

    pub fn color_at_object(&self, object: &Shape, world_point: &Point) -> Color {
        let color = match self.pattern {
            PatternKind::StripedPattern(s) => s.stripe_at_object(object, world_point),
            PatternKind::TestPattern(_) => unimplemented!(),
        };
        color.unwrap_or_default()
    }
}
