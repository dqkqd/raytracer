use serde::Deserialize;
use serde_yaml::Value;

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct VectorParser(f64, f64, f64);

#[allow(dead_code)]
impl VectorParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn to_vector(self) -> Vector {
        Vector::new(self.0, self.1, self.2)
    }

    pub fn from_value(value: Value) -> Option<Vector> {
        let parser: VectorParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_vector())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_vector() -> Vector {
        Vector::new(1.0, 2.0, 3.0)
    }

    fn default_parser() -> VectorParser {
        VectorParser(1.0, 2.0, 3.0)
    }

    #[test]
    fn parse_to_vector() {
        let vector = default_vector();
        let parser = default_parser();
        assert_eq!(parser.to_vector(), vector);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
[1.0, 2.0, 3.0]
";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let vector = VectorParser::from_value(value).unwrap();
        assert_eq!(vector, default_vector());
    }
}
