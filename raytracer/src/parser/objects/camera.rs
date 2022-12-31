use serde::Deserialize;
use serde_yaml::Value;

use crate::{
    camera::Camera,
    transform::{Transform, Transformable},
};

use super::{point::PointParser, vector::VectorParser};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct CameraParser {
    width: usize,
    height: usize,

    #[serde(rename(deserialize = "field-of-view"))]
    field_of_view: f64,

    from: PointParser,
    to: PointParser,
    up: VectorParser,
}

impl CameraParser {
    pub fn to_camera(self) -> Camera {
        let from = self.from.to_point();
        let to = self.to.to_point();
        let up = self.up.to_vector();
        let view_transform = Transform::view_transform(from, to, up);
        Camera::new(self.width, self.height, self.field_of_view).with_transform(view_transform)
    }

    pub(crate) fn from_value(value: Value) -> Option<Camera> {
        let parser: CameraParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_camera())
    }
}

#[cfg(test)]
mod test {

    use crate::{parser::yaml::Parser, point::Point, vector::Vector};

    use super::*;

    fn default_camera() -> Camera {
        let width = 10;
        let height = 20;
        let field_of_view = 1.25;

        let from = Point::new(1.0, 2.0, 3.0);
        let to = Point::new(4.0, 5.0, 6.0);
        let up = Vector::new(7.0, 8.0, 9.0);
        let view_transform = Transform::view_transform(from, to, up);

        Camera::new(width, height, field_of_view).with_transform(view_transform)
    }

    fn default_parser() -> CameraParser {
        CameraParser {
            width: 10,
            height: 20,
            field_of_view: 1.25,
            from: PointParser::new(1.0, 2.0, 3.0),
            to: PointParser::new(4.0, 5.0, 6.0),
            up: VectorParser::new(7.0, 8.0, 9.0),
        }
    }

    #[test]
    fn parse_to_camera() {
        let camera = default_camera();
        let parser = default_parser();
        assert_eq!(parser.to_camera(), camera);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
  width: 10
  height: 20
  field-of-view: 1.25
  from: [ 1, 2, 3 ]
  to: [ 4, 5, 6 ]
  up: [ 7, 8, 9 ]";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let camera = CameraParser::from_value(value).unwrap();
        assert_eq!(camera, default_camera());
    }

    #[test]
    fn parse_from_str() {
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
        let add_attributes = parser.add_attributes();
        assert_eq!(add_attributes.len(), 1);
        assert_eq!(add_attributes[0].attribute_type(), "camera");
        let camera = CameraParser::from_value(add_attributes[0].value()).unwrap();
        assert_eq!(camera, default_camera());
    }
}
