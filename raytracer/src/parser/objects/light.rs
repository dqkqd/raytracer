use serde::Deserialize;
use serde_yaml::Value;

use crate::{color::Color, light::PointLight, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct LightParser {
    #[serde(rename(deserialize = "at"))]
    position: [f64; 3],
    intensity: [f64; 3],
}

impl LightParser {
    pub fn to_light(self) -> PointLight {
        let position = Point::new(self.position[0], self.position[1], self.position[2]);
        let intensity = Color::new(self.intensity[0], self.intensity[1], self.intensity[2]);
        PointLight::new(position, intensity)
    }

    pub fn from_value(value: Value) -> Option<PointLight> {
        let parser: LightParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_light())
    }
}

#[cfg(test)]
mod test {

    use crate::parser::yaml::Parser;

    use super::*;

    fn default_point_light() -> PointLight {
        let position = Point::new(1.0, 2.0, 3.0);
        let intensity = Color::new(0.4, 0.5, 0.6);
        PointLight::new(position, intensity)
    }

    fn default_parser() -> LightParser {
        LightParser {
            position: [1.0, 2.0, 3.0],
            intensity: [0.4, 0.5, 0.6],
        }
    }

    #[test]
    fn parse_to_point_light() {
        let camera = default_point_light();
        let parser = default_parser();
        assert_eq!(parser.to_light(), camera);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
at: [1.0, 2.0, 3.0]
intensity: [0.4, 0.5, 0.6]
";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let light = LightParser::from_value(value).unwrap();
        assert_eq!(light, default_point_light());
    }

    #[test]
    fn parse_from_str() {
        let yaml = "
- add : light
  at: [1.0, 2.0, 3.0]
  intensity: [0.4, 0.5, 0.6]
";
        let parser = Parser::from_yaml(yaml).unwrap();
        let add_attributes = parser.add_attributes();
        assert_eq!(add_attributes.len(), 1);
        assert_eq!(add_attributes[0].attribute_type(), "light");
        let light = LightParser::from_value(add_attributes[0].value()).unwrap();
        assert_eq!(light, default_point_light());
    }
}
