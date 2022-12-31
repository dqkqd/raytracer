use crate::{color::Color, point::Point, shapes::shape::Shape};

use self::{
    checker::CheckerPattern, dummy::DummyPattern, gradient::GradientPattern, ring::RingPattern,
    stripe::StripedPattern,
};

pub(crate) mod pattern;

pub(crate) mod dummy;

pub(crate) mod stripe;

pub(crate) mod gradient;

pub(crate) mod ring;

pub(crate) mod checker;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PatternKind {
    Stripe(StripedPattern),
    Dummy(DummyPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
}

pub(crate) trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub(crate) trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
