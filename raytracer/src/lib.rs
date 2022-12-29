pub(crate) mod util;

pub(crate) mod point;
pub use point::Point;

pub(crate) mod vector;
pub use vector::Vector;

pub mod color;
pub use color::Color;

pub(crate) mod canvas;
pub use canvas::Canvas;

pub(crate) mod matrix;

pub(crate) mod transform;
pub(crate) use transform::InversedTransform;
pub use transform::{Transform, Transformable};

pub(crate) mod ray;
pub use ray::Ray;

pub mod shapes;
pub use shapes::shape::Shape;

pub(crate) mod intersect;
pub(crate) use intersect::{Intersections, IntersectionsFactor};

pub(crate) mod light;
pub use light::PointLight;

pub(crate) mod phong;

pub(crate) mod material;
pub use material::Material;

pub mod world;
pub use world::World;

pub mod camera;
pub use camera::Camera;

pub mod patterns;
pub use patterns::pattern::Pattern;
