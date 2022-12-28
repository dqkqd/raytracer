use crate::{Color, Point, Shape};

pub mod stripe;

pub mod pattern;

pub mod dummy_pattern;

pub trait PatternLocal {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternWorld: PatternLocal {
    fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color;
}
