use serde::Deserialize;
use serde_yaml::Value;

use crate::{color::Color, material::Material};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct MaterialParser {
    color: [f64; 3],
    diffuse: f64,
    ambient: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,

    #[serde(rename(deserialize = "refractive-index"))]
    refractive_index: f64,
}

#[allow(dead_code)]
impl MaterialParser {
    pub fn to_material(self) -> Material {
        Material::default()
            .with_color(Color::new(self.color[0], self.color[1], self.color[2]))
            .with_diffuse(self.diffuse)
            .with_ambient(self.ambient)
            .with_specular(self.specular)
            .with_shininess(self.shininess)
            .with_reflective(self.reflective)
            .with_transparency(self.transparency)
            .with_refractive_index(self.refractive_index)
    }

    pub fn from_value(value: Value) -> Option<Material> {
        let parser: MaterialParser = serde_yaml::from_value(value).ok()?;
        Some(parser.to_material())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        color: [f64; 3],
        diffuse: f64,
        ambient: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> MaterialParser {
        MaterialParser {
            color,
            diffuse,
            ambient,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_material() -> Material {
        Material::default()
            .with_color(Color::new(0.1, 0.2, 0.3))
            .with_diffuse(0.4)
            .with_ambient(0.5)
            .with_specular(0.6)
            .with_shininess(0.7)
            .with_reflective(0.8)
            .with_transparency(0.9)
            .with_refractive_index(1.3)
    }

    fn default_parser() -> MaterialParser {
        MaterialParser {
            color: [0.1, 0.2, 0.3],
            diffuse: 0.4,
            ambient: 0.5,
            specular: 0.6,
            shininess: 0.7,
            reflective: 0.8,
            transparency: 0.9,
            refractive_index: 1.3,
        }
    }

    #[test]
    fn parse_to_point_light() {
        let camera = default_material();
        let parser = default_parser();
        assert_eq!(parser.to_material(), camera);
    }

    #[test]
    fn parse_from_value() {
        let yaml = "
color: [0.1, 0.2, 0.3]
diffuse: 0.4
ambient: 0.5
specular: 0.6
shininess: 0.7
reflective: 0.8
transparency: 0.9
refractive-index: 1.3
";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let material = MaterialParser::from_value(value).unwrap();
        assert_eq!(material, default_material());
    }
}
