use crate::{Color, Point, Shape};

pub mod pattern;

pub mod stripe;

pub mod gradient;

pub mod dummy_pattern;

pub trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
