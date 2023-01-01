use serde::Deserialize;

use crate::light::PointLight;

use super::{color::ColorParser, point::PointParser, ObjectParser};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct LightParser {
    at: PointParser,
    intensity: ColorParser,
}

impl ObjectParser<PointLight> for LightParser {
    fn parse(&self) -> PointLight {
        let position = self.at.parse();
        let intensity = self.intensity.parse();
        PointLight::new(position, intensity)
    }
}

#[cfg(test)]
mod test {

    use serde_yaml::Value;

    use crate::{color::Color, parser::yaml::Parser, point::Point};

    use super::*;

    fn default_point_light() -> PointLight {
        let position = Point::new(1.0, 2.0, 3.0);
        let intensity = Color::new(0.4, 0.5, 0.6);
        PointLight::new(position, intensity)
    }

    fn default_parser() -> LightParser {
        LightParser {
            at: PointParser::new(1.0, 2.0, 3.0),
            intensity: ColorParser::new(0.4, 0.5, 0.6),
        }
    }

    #[test]
    fn parse_to_point_light() {
        let light = default_point_light();
        let parser = default_parser();
        assert_eq!(parser.parse(), light);
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
