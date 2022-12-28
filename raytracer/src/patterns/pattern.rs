use crate::{Color, Point};

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
}
