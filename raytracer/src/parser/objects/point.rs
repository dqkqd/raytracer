use serde::Deserialize;
use serde_yaml::Value;

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct PointParser(f64, f64, f64);

#[allow(dead_code)]
impl PointParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn to_point(self) -> Point {
        Point::new(self.0, self.1, self.2)
    }

    pub fn from_value(value: Value) -> Option<Point> {
        let parser: PointParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_point())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_point() -> Point {
        Point::new(1.0, 2.0, 3.0)
    }

    fn default_parser() -> PointParser {
        PointParser(1.0, 2.0, 3.0)
    }

    #[test]
    fn parse_to_point() {
        let camera = default_point();
        let parser = default_parser();
        assert_eq!(parser.to_point(), camera);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
[1.0, 2.0, 3.0]
";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let light = PointParser::from_value(value).unwrap();
        assert_eq!(light, default_point());
    }
}
