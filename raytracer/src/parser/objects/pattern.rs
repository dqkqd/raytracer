use serde::Deserialize;
use serde_yaml::Value;

use crate::{patterns::pattern::Pattern, transform::Transformable};

use super::{color::ColorParser, transform::TransformParser};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct PatternParser {
    #[serde(rename(deserialize = "type"))]
    pattern_type: String,

    colors: Vec<ColorParser>,

    transform: TransformParser,
}

#[allow(dead_code)]
impl PatternParser {
    pub fn to_pattern(&self) -> Pattern {
        assert_eq!(self.colors.len(), 2, "Only support pattern with 2 colors");
        let left_color = self.colors[0].to_color();
        let right_color = self.colors[1].to_color();
        let pattern = match self.pattern_type.as_str() {
            "stripes" => Pattern::stripe(left_color, right_color),
            "checkers" => Pattern::checker(left_color, right_color),
            _ => unimplemented!(),
        };
        let transform = self.transform.to_transform();
        pattern.with_transform(transform)
    }

    pub fn from_value(value: Value) -> Option<Pattern> {
        let parser: PatternParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_pattern())
    }
}

#[cfg(test)]
mod test {

    use crate::{
        color::Color, parser::objects::transform::SingleTransformParser, transform::Transform,
    };

    use super::*;

    fn default_pattern() -> Pattern {
        let left_color = Color::new(0.1, 0.2, 0.3);
        let right_color = Color::new(0.4, 0.5, 0.6);
        let transform = Transform::translation(1.0, 2.0, 3.0)
            .scale(0.4, 0.5, 0.6)
            .rotate_z(0.5);
        Pattern::stripe(left_color, right_color).with_transform(transform)
    }

    fn default_parser() -> PatternParser {
        let transform = TransformParser::new(vec![
            SingleTransformParser::TranslationScaling("translate".to_string(), 1.0, 2.0, 3.0),
            SingleTransformParser::TranslationScaling("scale".to_string(), 0.4, 0.5, 0.6),
            SingleTransformParser::Rotation("rotate-z".to_string(), 0.5),
        ]);
        PatternParser {
            pattern_type: "stripes".to_string(),
            colors: vec![
                ColorParser::new(0.1, 0.2, 0.3),
                ColorParser::new(0.4, 0.5, 0.6),
            ],
            transform,
        }
    }

    fn default_yaml() -> String {
        String::from(
            "
type: stripes
colors:
- [0.1, 0.2, 0.3]
- [0.4, 0.5, 0.6]
transform:
- [translate, 1, 2, 3]
- [scale, 0.4, 0.5, 0.6]
- [rotate-z, 0.5]
        ",
        )
    }

    #[test]
    fn parse_to_pattern() {
        let pattern = default_pattern();
        let parser = default_parser();
        assert_eq!(parser.to_pattern(), pattern);
    }

    #[test]
    fn parse_from_value() {
        let value: Value = serde_yaml::from_str(&default_yaml()).unwrap();
        let pattern = PatternParser::from_value(value).unwrap();
        assert_eq!(pattern, default_pattern());
    }
}
