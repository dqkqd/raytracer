use crate::{Color, Point, Shape};

use self::{dummy_pattern::TestPattern, gradient::GradientPattern, stripe::StripedPattern};

pub mod pattern;

pub mod stripe;

pub mod gradient;

pub mod dummy_pattern;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
    TestPattern(TestPattern),
    GradientPattern(GradientPattern),
}

pub trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
