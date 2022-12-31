use serde::Deserialize;
use serde_yaml::Value;

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct ColorParser(f64, f64, f64);

#[allow(dead_code)]
impl ColorParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn to_color(self) -> Color {
        Color::new(self.0, self.1, self.2)
    }

    pub fn from_value(value: Value) -> Option<Color> {
        let parser: ColorParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_color())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_color() -> Color {
        Color::new(0.1, 0.2, 0.3)
    }

    fn default_parser() -> ColorParser {
        ColorParser(0.1, 0.2, 0.3)
    }

    #[test]
    fn parse_to_color() {
        let camera = default_color();
        let parser = default_parser();
        assert_eq!(parser.to_color(), camera);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
[0.1, 0.2, 0.3]
";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let light = ColorParser::from_value(value).unwrap();
        assert_eq!(light, default_color());
    }
}