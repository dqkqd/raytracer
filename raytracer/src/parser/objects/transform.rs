use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::transform::Transform;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum TransformParser {
    Rotation(String, f64),
    TranslationScaling(String, f64, f64, f64),
    Shearing(String, f64, f64, f64, f64, f64, f64),
}

#[allow(dead_code)]
impl TransformParser {
    pub fn to_transform(&self) -> Transform {
        match self {
            TransformParser::Rotation(s, a) => match s.as_str() {
                "rotate-x" => Transform::rotation_x(*a),
                "rotate-y" => Transform::rotation_y(*a),
                "rotate-z" => Transform::rotation_z(*a),
                _ => unimplemented!(),
            },
            TransformParser::TranslationScaling(s, x, y, z) => match s.as_str() {
                "translate" => Transform::translation(*x, *y, *z),
                "scale" => Transform::scaling(*x, *y, *z),
                _ => unimplemented!(),
            },
            TransformParser::Shearing(s, xy, xz, yx, yz, zx, zy) => match s.as_str() {
                "shear" => Transform::shearing(*xy, *xz, *yx, *yz, *zx, *zy),
                _ => unimplemented!(),
            },
        }
    }

    pub fn from_value(value: Value) -> Option<Transform> {
        let parser: TransformParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_transform())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_to_rotate() {
        let transform = Transform::rotation_y(1.5);
        let parser = TransformParser::Rotation("rotate-y".to_string(), 1.5);
        assert_eq!(parser.to_transform(), transform);
    }

    #[test]
    fn parse_rotate_value() {
        let yaml = "[rotate-y, 1.5]";
        let transform = Transform::rotation_y(1.5);
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let expected = TransformParser::from_value(value).unwrap();
        assert_eq!(expected, transform);
    }

    #[test]
    fn parse_to_translate() {
        let transform = Transform::translation(1.5, 2.5, 3.5);
        let parser = TransformParser::TranslationScaling("translate".to_string(), 1.5, 2.5, 3.5);
        assert_eq!(parser.to_transform(), transform);
    }

    #[test]
    fn parse_translate_value() {
        let yaml = "[translate, 1.5, 2.5, 3.5]";
        let transform = Transform::translation(1.5, 2.5, 3.5);
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let expected = TransformParser::from_value(value).unwrap();
        assert_eq!(expected, transform);
    }

    #[test]
    fn parse_to_scale() {
        let transform = Transform::scaling(1.5, 2.5, 3.5);
        let parser = TransformParser::TranslationScaling("scale".to_string(), 1.5, 2.5, 3.5);
        assert_eq!(parser.to_transform(), transform);
    }

    #[test]
    fn parse_scale_value() {
        let yaml = "[scale, 1.5, 2.5, 3.5]";
        let transform = Transform::scaling(1.5, 2.5, 3.5);
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let expected = TransformParser::from_value(value).unwrap();
        assert_eq!(expected, transform);
    }

    #[test]
    fn parse_to_shear() {
        let transform = Transform::shearing(1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        let parser = TransformParser::Shearing("shear".to_string(), 1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        assert_eq!(parser.to_transform(), transform);
    }

    #[test]
    fn parse_shear_value() {
        let yaml = "[shear, 1.5, 2.5, 3.5, 7.5, 6.4, -5.3]";
        let transform = Transform::shearing(1.5, 2.5, 3.5, 7.5, 6.4, -5.3);
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let expected = TransformParser::from_value(value).unwrap();
        assert_eq!(expected, transform);
    }
}
