use crate::{
    camera::Camera, light::PointLight, parser::attributes::add_attribute::AddAttribute,
    shapes::shape::Shape,
};

use super::{
    camera::CameraParser, light::LightParser, shape::ShapeParser, ObjectParser, ParseResult,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Object {
    Camera(Camera),
    Light(PointLight),
    Shape(Shape),
}

#[allow(dead_code)]
impl Object {
    pub fn from_attribute(attr: &AddAttribute) -> ParseResult<Object> {
        let value = attr.value();
        let attribute_type = attr.attribute_type();
        match attribute_type {
            "camera" => Ok(Object::Camera(CameraParser::from_value(value)?)),
            "light" => Ok(Object::Light(LightParser::from_value(value)?)),
            "sphere" | "plane" | "cube" | "cylinder" | "cone" => Ok(Object::Shape(
                ShapeParser::from_value(value, attribute_type)?,
            )),
            s => unimplemented!("Parser for `{}` is not implemented", s),
        }
    }
    pub fn as_camera(&self) -> Option<&Camera> {
        match self {
            Object::Camera(camera) => Some(camera),
            _ => None,
        }
    }

    pub fn as_light(&self) -> Option<&PointLight> {
        match self {
            Object::Light(light) => Some(light),
            _ => None,
        }
    }

    pub fn as_shape(&self) -> Option<&Shape> {
        match self {
            Object::Shape(shape) => Some(shape),
            _ => None,
        }
    }
}
