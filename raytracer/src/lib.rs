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
pub use transform::Transform;

pub(crate) mod ray;
pub use ray::Ray;

pub(crate) mod shapes;
pub use shapes::object;
pub use shapes::shape::Shape;
pub use shapes::sphere::Sphere;

pub(crate) mod intersection;
pub use intersection::Intersections;

pub(crate) mod light;
pub use light::PointLight;

pub(crate) mod phong;

pub(crate) mod material;
pub use material::Material;

pub mod world;
pub use world::World;
