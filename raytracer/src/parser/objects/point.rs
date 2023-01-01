use serde::Deserialize;

use crate::point::Point;

use super::ObjectParser;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct PointParser(f64, f64, f64);

#[allow(dead_code)]
impl PointParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl ObjectParser<Point> for PointParser {
    fn parse(&self) -> Point {
        Point::new(self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod test {

    use serde_yaml::Value;

    use crate::parser::objects::ParseResult;

    use super::*;

    fn default_point() -> Point {
        Point::new(1.0, 2.0, 3.0)
    }

    fn default_parser() -> PointParser {
        PointParser(1.0, 2.0, 3.0)
    }

    #[test]
    fn parse_to_point() {
        let point = default_point();
        let parser = default_parser();
        assert_eq!(parser.parse(), point);
    }

    #[test]
    fn parse_from_value() -> ParseResult<()> {
        let yaml = "
[1.0, 2.0, 3.0]
";
        let value: Value = serde_yaml::from_str(yaml)?;
        let point = PointParser::from_value(value)?;
        assert_eq!(point, default_point());
        Ok(())
    }
}
