use crate::{parser::yaml::AddAttribute, Camera};

use super::camera::CameraParser;

#[derive(Debug, Clone, Copy, PartialEq)]

pub(crate) enum Object {
    Camera(Camera),
}

impl Object {
    pub(crate) fn from_attribute(attr: &AddAttribute) -> Option<Object> {
        let value = attr.value()?;
        let attribute_type = attr.attribute_type();
        match attribute_type {
            "camera" => Some(Object::Camera(CameraParser::from_value(value)?)),
            _ => unimplemented!(),
        }
    }
    fn as_camera(&self) -> Option<&Camera> {
        match self {
            Object::Camera(camera) => Some(camera),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::yaml::Parser;

    use super::*;

    #[test]
    fn parse_object_from_attribute() {
        let yaml = "
- add : camera
  width: 10
  height: 20
  field-of-view: 1.25
  from: [ 1, 2, 3 ]
  to: [ 4, 5, 6 ]
  up: [ 7, 8, 9 ]
";
        let parser = Parser::from_yaml(yaml).unwrap();
        let attr = &parser.add_attributes()[0];
        let object = Object::from_attribute(attr).unwrap();
        assert!(object.as_camera().is_some());
    }
}
