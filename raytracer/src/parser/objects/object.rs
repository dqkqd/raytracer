use crate::{parser::yaml::AddAttribute, Camera};

use super::camera::CameraParser;

#[derive(Debug, Clone, Copy, PartialEq)]

pub(crate) enum Object {
    Camera(Camera),
}

impl Object {
    pub fn from_attribute(attr: &AddAttribute) -> Option<Object> {
        let value = attr.value()?;
        let attribute_type = attr.attribute_type();
        match attribute_type {
            "camera" => Some(Object::Camera(CameraParser::from_value(value)?)),
            _ => unimplemented!(),
        }
    }
    pub fn as_camera(&self) -> Option<&Camera> {
        match self {
            Object::Camera(camera) => Some(camera),
            _ => None,
        }
    }
}
