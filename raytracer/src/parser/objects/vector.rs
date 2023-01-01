use serde::Deserialize;

use crate::vector::Vector;

use super::ObjectParser;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct VectorParser(f64, f64, f64);

#[allow(dead_code)]
impl VectorParser {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl ObjectParser<Vector> for VectorParser {
    fn parse(&self) -> Vector {
        Vector::new(self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod test {

    use serde_yaml::Value;

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
        assert_eq!(parser.parse(), vector);
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
