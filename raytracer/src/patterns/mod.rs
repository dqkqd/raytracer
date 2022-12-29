use crate::{Color, Point, Shape};

use self::{
    checker::CheckerPattern, dummy::DummyPattern, gradient::GradientPattern, ring::RingPattern,
    stripe::StripedPattern,
};

pub mod pattern;

pub mod dummy;

pub mod stripe;

pub mod gradient;

pub mod ring;

pub mod checker;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PatternKind {
    Stripe(StripedPattern),
    Dummy(DummyPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
}

pub trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
