pub(crate) mod util;

pub(crate) mod point;

pub(crate) mod vector;

pub(crate) mod color;

pub(crate) mod canvas;

pub(crate) mod matrix;

pub(crate) mod transform;

pub(crate) mod ray;

pub(crate) mod shapes;

pub(crate) mod intersect;

pub(crate) mod light;

pub(crate) mod phong;

pub(crate) mod material;

pub(crate) mod world;

pub(crate) mod camera;

pub(crate) mod patterns;

pub(crate) mod parser;

pub use parser::render::render_image;
