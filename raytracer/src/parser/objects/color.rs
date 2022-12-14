use serde::Deserialize;

use crate::color::Color;

use super::ObjectParser;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct ColorParser(f64, f64, f64);

#[allow(dead_code)]
impl ColorParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl ObjectParser<Color> for ColorParser {
    fn parse(&self) -> Color {
        Color::new(self.0, self.1, self.2)
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        let color = Color::default();
        Self(color.r(), color.g(), color.b())
    }
}

#[cfg(test)]
mod test {

    use serde_yaml::Value;

    use crate::parser::objects::ParseResult;

    use super::*;

    fn default_color() -> Color {
        Color::new(0.1, 0.2, 0.3)
    }

    fn default_parser() -> ColorParser {
        ColorParser(0.1, 0.2, 0.3)
    }

    #[test]
    fn parse_to_color() {
        let color = default_color();
        let parser = default_parser();
        assert_eq!(parser.parse(), color);
    }

    #[test]
    fn parse_from_value() -> ParseResult<()> {
        let yaml = "
[0.1, 0.2, 0.3]
";
        let value: Value = serde_yaml::from_str(yaml)?;
        let color = ColorParser::from_value(value)?;
        assert_eq!(color, default_color());
        Ok(())
    }
}
