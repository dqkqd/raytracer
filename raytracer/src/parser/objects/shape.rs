use serde::Deserialize;
use serde_yaml::Value;

use crate::{shapes::ShapeMaterial, Shape};

use super::material::MaterialParser;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct ShapeParser {
    #[serde(rename(deserialize = "material"))]
    material_parser: MaterialParser,
}

impl ShapeParser {
    pub fn to_shape(self, shape_type: &str) -> Shape {
        let shape = match shape_type {
            "sphere" => Shape::sphere(),
            "plane" => Shape::plane(),
            "cube" => Shape::cube(),
            _ => unimplemented!(),
        };
        let material = self.material_parser.to_material();
        shape.with_material(material)
    }

    pub fn from_value(value: Value, attribute_type: &str) -> Option<Shape> {
        let parser: ShapeParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_shape(attribute_type))
    }
}

#[cfg(test)]
mod test {

    use crate::{parser::yaml::Parser, Color, Material};

    use super::*;

    fn default_object_without_transform() -> (String, Shape) {
        let material = Material::default()
            .with_color(Color::new(0.1, 0.2, 0.3))
            .with_diffuse(0.4)
            .with_ambient(0.5)
            .with_specular(0.6)
            .with_shininess(0.7)
            .with_reflective(0.8)
            .with_transparency(0.9)
            .with_refractive_index(1.3);
        (
            "sphere".to_string(),
            Shape::sphere().with_material(material),
        )
    }

    fn default_parser_without_transform() -> ShapeParser {
        let material_parser =
            MaterialParser::new([0.1, 0.2, 0.3], 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.3);
        ShapeParser { material_parser }
    }

    #[test]
    fn parse_to_shape() {
        let (shape_type, shape) = default_object_without_transform();
        let parser = default_parser_without_transform();
        assert_eq!(parser.to_shape(&shape_type), shape);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
material:
  color: [0.1, 0.2, 0.3]
  diffuse: 0.4
  ambient: 0.5
  specular: 0.6
  shininess: 0.7
  reflective: 0.8
  transparency: 0.9
  refractive_index: 1.3
    ";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let (shape_type, default_shape) = default_object_without_transform();
        let shape = ShapeParser::from_value(value, &shape_type).unwrap();
        assert_eq!(shape, default_shape);
    }

    #[test]
    fn parse_from_str() {
        let yaml = "
- add: sphere
  material:
    color: [0.1, 0.2, 0.3]
    diffuse: 0.4
    ambient: 0.5
    specular: 0.6
    shininess: 0.7
    reflective: 0.8
    transparency: 0.9
    refractive_index: 1.3
    ";
        let parser = Parser::from_yaml(yaml).unwrap();
        let add_attributes = parser.add_attributes();
        let (shape_type, default_shape) = default_object_without_transform();
        assert_eq!(add_attributes.len(), 1);
        assert_eq!(add_attributes[0].attribute_type(), shape_type);
        let shape =
            ShapeParser::from_value(add_attributes[0].value().unwrap(), &shape_type).unwrap();
        assert_eq!(shape, default_shape);
    }
}
