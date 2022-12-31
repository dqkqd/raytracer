use serde::Deserialize;
use serde_yaml::Value;

use crate::material::Material;

use super::{color::ColorParser, pattern::PatternParser};

fn default_color() -> ColorParser {
    let color = Material::default().color();
    ColorParser::new(color.r(), color.g(), color.b())
}

fn default_diffuse() -> f64 {
    Material::default().diffuse()
}

fn default_ambient() -> f64 {
    Material::default().ambient()
}

fn default_specular() -> f64 {
    Material::default().specular()
}

fn default_shininess() -> f64 {
    Material::default().shininess()
}

fn default_reflective() -> f64 {
    Material::default().reflective()
}

fn default_transparency() -> f64 {
    Material::default().transparency()
}

fn default_refractive_index() -> f64 {
    Material::default().refractive_index()
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct MaterialParser {
    #[serde(default = "default_color")]
    color: ColorParser,

    #[serde(default = "default_diffuse")]
    diffuse: f64,

    #[serde(default = "default_ambient")]
    ambient: f64,

    #[serde(default = "default_specular")]
    specular: f64,

    #[serde(default = "default_shininess")]
    shininess: f64,

    #[serde(default = "default_reflective")]
    reflective: f64,

    #[serde(default = "default_transparency")]
    transparency: f64,

    #[serde(
        rename(deserialize = "refractive-index"),
        default = "default_refractive_index"
    )]
    refractive_index: f64,

    #[serde(default)]
    pattern: Option<PatternParser>,
}

impl Default for MaterialParser {
    fn default() -> Self {
        let material = Material::default();
        let material_color = material.color();
        let color = ColorParser::new(material_color.r(), material_color.g(), material_color.b());
        MaterialParser {
            color,
            diffuse: material.diffuse(),
            ambient: material.ambient(),
            specular: material.specular(),
            shininess: material.shininess(),
            reflective: material.reflective(),
            transparency: material.transparency(),
            refractive_index: material.refractive_index(),
            pattern: None,
        }
    }
}

#[allow(dead_code)]
impl MaterialParser {
    pub fn to_material(&self) -> Material {
        let material = Material::default()
            .with_color(self.color.to_color())
            .with_diffuse(self.diffuse)
            .with_ambient(self.ambient)
            .with_specular(self.specular)
            .with_shininess(self.shininess)
            .with_reflective(self.reflective)
            .with_transparency(self.transparency)
            .with_refractive_index(self.refractive_index);
        match &self.pattern {
            Some(p) => material.with_pattern(p.to_pattern()),
            _ => material,
        }
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
            color: ColorParser::new(color[0], color[1], color[2]),
            diffuse,
            ambient,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
            pattern: None,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        color::Color,
        patterns::pattern::Pattern,
        transform::{Transform, Transformable},
    };

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
            color: ColorParser::new(0.1, 0.2, 0.3),
            diffuse: 0.4,
            ambient: 0.5,
            specular: 0.6,
            shininess: 0.7,
            reflective: 0.8,
            transparency: 0.9,
            refractive_index: 1.3,
            pattern: None,
        }
    }

    #[test]
    fn parse_to_material() {
        let material = default_material();
        let parser = default_parser();
        assert_eq!(parser.to_material(), material);
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

    #[test]
    fn parse_from_value_with_pattern() {
        let yaml = "
color: [0.1, 0.2, 0.3]
refractive-index: 1.3
pattern:
  type: stripes
  colors:
    - [0.1, 0.2, 0.3]
    - [0.4, 0.5, 0.6]
  transform:
    - [translate, 1, 2, 3]
    - [scale, 0.4, 0.5, 0.6]
    - [rotate-z, 0.5]
";

        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let material = MaterialParser::from_value(value);

        let pattern = Pattern::stripe(Color::new(0.1, 0.2, 0.3), Color::new(0.4, 0.5, 0.6))
            .with_transform(
                Transform::translation(1.0, 2.0, 3.0)
                    .scale(0.4, 0.5, 0.6)
                    .rotate_z(0.5),
            );
        let expected = Material::default()
            .with_color(Color::new(0.1, 0.2, 0.3))
            .with_refractive_index(1.3)
            .with_pattern(pattern);

        assert_eq!(material, Some(expected));
    }
}
