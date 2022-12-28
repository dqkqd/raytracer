use crate::{Color, Point, Shape};

use self::{
    dummy_pattern::TestPattern, gradient::GradientPattern, ring::RingPattern,
    stripe::StripedPattern,
};

pub mod pattern;

pub mod dummy_pattern;

pub mod stripe;

pub mod gradient;

pub mod ring;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
    TestPattern(TestPattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
}

pub trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
