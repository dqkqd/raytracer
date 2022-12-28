use crate::{Color, Point, Shape};

use super::stripe::StripedPattern;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
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
        }
    }

    pub fn color_at_object(&self, object: &Shape, world_point: &Point) -> Color {
        let color = match self.pattern {
            PatternKind::StripedPattern(s) => s.stripe_at_object(object, world_point),
        };
        color.unwrap_or_default()
    }
}
