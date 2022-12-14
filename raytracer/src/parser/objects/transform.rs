use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::transform::Transform;

use super::ObjectParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum SingleTransformParser {
    Rotation(String, f64),
    TranslationScaling(String, f64, f64, f64),
    Shearing(String, f64, f64, f64, f64, f64, f64),
}

#[allow(dead_code)]
impl SingleTransformParser {
    pub fn parse(&self) -> Transform {
        match self {
            SingleTransformParser::Rotation(s, a) => match s.as_str() {
                "rotate-x" => Transform::rotation_x(*a),
                "rotate-y" => Transform::rotation_y(*a),
                "rotate-z" => Transform::rotation_z(*a),
                _ => unimplemented!(),
            },
            SingleTransformParser::TranslationScaling(s, x, y, z) => match s.as_str() {
                "translate" => Transform::translation(*x, *y, *z),
                "scale" => Transform::scaling(*x, *y, *z),
                _ => unimplemented!(),
            },
            SingleTransformParser::Shearing(s, xy, xz, yx, yz, zx, zy) => match s.as_str() {
                "shear" => Transform::shearing(*xy, *xz, *yx, *yz, *zx, *zy),
                _ => unimplemented!(),
            },
        }
    }

    pub fn from_value(value: Value) -> Result<Transform, serde_yaml::Error> {
        let parser: SingleTransformParser = serde_yaml::from_value(value)?;
        Ok(parser.parse())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub(crate) struct TransformParser(Vec<SingleTransformParser>);

#[allow(dead_code)]
impl TransformParser {
    pub fn new(transform_list: Vec<SingleTransformParser>) -> Self {
        Self(transform_list)
    }
}
impl ObjectParser<Transform> for TransformParser {
    fn parse(&self) -> Transform {
        self.0
            .iter()
            .rev()
            .map(|t| t.parse())
            .reduce(|acc, t| acc * t)
            .unwrap_or_else(Transform::identity)
    }
}

#[cfg(test)]
mod test {

    use crate::parser::objects::ParseResult;

    use super::*;

    #[test]
    fn parse_to_rotate() {
        let transform = Transform::rotation_y(1.5);
        let parser = SingleTransformParser::Rotation("rotate-y".to_string(), 1.5);
        assert_eq!(parser.parse(), transform);
    }

    #[test]
    fn parse_rotate_value() -> ParseResult<()> {
        let yaml = "[rotate-y, 1.5]";
        let transform = Transform::rotation_y(1.5);
        let value: Value = serde_yaml::from_str(yaml)?;
        let expected = SingleTransformParser::from_value(value)?;
        assert_eq!(expected, transform);
        Ok(())
    }

    #[test]
    fn parse_to_translate() {
        let transform = Transform::translation(1.5, 2.5, 3.5);
        let parser =
            SingleTransformParser::TranslationScaling("translate".to_string(), 1.5, 2.5, 3.5);
        assert_eq!(parser.parse(), transform);
    }

    #[test]
    fn parse_translate_value() -> ParseResult<()> {
        let yaml = "[translate, 1.5, 2.5, 3.5]";
        let transform = Transform::translation(1.5, 2.5, 3.5);
        let value: Value = serde_yaml::from_str(yaml)?;
        let expected = SingleTransformParser::from_value(value)?;
        assert_eq!(expected, transform);
        Ok(())
    }

    #[test]
    fn parse_to_scale() {
        let transform = Transform::scaling(1.5, 2.5, 3.5);
        let parser = SingleTransformParser::TranslationScaling("scale".to_string(), 1.5, 2.5, 3.5);
        assert_eq!(parser.parse(), transform);
    }

    #[test]
    fn parse_scale_value() -> ParseResult<()> {
        let yaml = "[scale, 1.5, 2.5, 3.5]";
        let transform = Transform::scaling(1.5, 2.5, 3.5);
        let value: Value = serde_yaml::from_str(yaml)?;
        let expected = SingleTransformParser::from_value(value)?;
        assert_eq!(expected, transform);
        Ok(())
    }

    #[test]
    fn parse_to_shear() {
        let transform = Transform::shearing(1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        let parser =
            SingleTransformParser::Shearing("shear".to_string(), 1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        assert_eq!(parser.parse(), transform);
    }

    #[test]
    fn parse_shear_value() -> ParseResult<()> {
        let yaml = "[shear, 1.5, 2.5, 3.5, 7.5, 6.4, -5.3]";
        let transform = Transform::shearing(1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        let value: Value = serde_yaml::from_str(yaml)?;
        let expected = SingleTransformParser::from_value(value)?;
        assert_eq!(expected, transform);
        Ok(())
    }

    fn default_transform() -> Transform {
        Transform::rotation_x(1.5)
            .translate(1.0, 2.0, 3.0)
            .scale(4.0, 5.0, 6.0)
            .shear(7.0, 8.0, 9.0, 10.0, 11.0, 12.5)
    }

    fn default_parser() -> TransformParser {
        TransformParser(vec![
            SingleTransformParser::Rotation("rotate-x".to_string(), 1.5),
            SingleTransformParser::TranslationScaling("translate".to_string(), 1.0, 2.0, 3.0),
            SingleTransformParser::TranslationScaling("scale".to_string(), 4.0, 5.0, 6.0),
            SingleTransformParser::Shearing("shear".to_string(), 7.0, 8.0, 9.0, 10.0, 11.0, 12.5),
        ])
    }

    fn default_yaml() -> String {
        String::from(
            "
- ['rotate-x', 1.5]
- ['translate', 1, 2, 3]
- ['scale', 4, 5, 6]
- ['shear', 7, 8, 9, 10, 11, 12.5]
",
        )
    }

    #[test]
    fn parser_to_combined_transform() {
        let transform = default_transform();
        let parser = default_parser();
        assert_eq!(parser.parse(), transform);
    }

    #[test]
    fn parse_combined_transform_from_value() -> ParseResult<()> {
        let value: Value = serde_yaml::from_str(&default_yaml())?;
        let transform = TransformParser::from_value(value)?;
        assert_eq!(transform, default_transform());
        Ok(())
    }
}
