use std::{collections::HashMap, fs};

use serde_yaml::Value;

use super::{
    attributes::{add_attribute::AddAttribute, define_attribute::DefineAttribute},
    objects::object::Object,
};

pub(crate) fn from_str(yaml_str: &str) -> Option<Vec<Object>> {
    Parser::from_yaml(yaml_str)?
        .add_attributes()
        .iter()
        .map(|attr| attr.parse())
        .collect::<Option<Vec<Object>>>()
}

pub(crate) fn from_file(file_name: &str) -> Option<Vec<Object>> {
    let yaml_str = fs::read_to_string(file_name).ok()?;
    from_str(&yaml_str)
}

pub(crate) type DefineAttributes = HashMap<String, DefineAttribute>;

#[derive(Debug, Default)]
pub(crate) struct Parser {
    add_attributes: Vec<AddAttribute>,
    define_attributes: DefineAttributes,
}

impl Parser {
    pub(crate) fn from_yaml(yaml: &str) -> Option<Parser> {
        let mut parser = Parser::from_yaml_without_preprocessing(yaml)?;
        parser.prepare();
        Some(parser)
    }

    pub(crate) fn add_attributes(&self) -> &Vec<AddAttribute> {
        &self.add_attributes
    }

    fn from_yaml_without_preprocessing(yaml: &str) -> Option<Parser> {
        let mut add_attributes = Vec::new();
        let mut define_attributes = HashMap::new();
        if let Ok(yaml_value) = serde_yaml::from_str(yaml) {
            let values: Value = yaml_value;
            let seq = values.as_sequence()?;

            for value in seq {
                let mapping = value.as_mapping()?;
                if mapping.contains_key("add") {
                    add_attributes.push(AddAttribute::new(value.clone()));
                } else if mapping.contains_key("define") {
                    let define_id = mapping.get("define")?.as_str()?.to_string();
                    define_attributes.insert(define_id, DefineAttribute::new(value.clone()));
                }
            }
        }

        Some(Parser {
            add_attributes,
            define_attributes,
        })
    }

    fn extend(&mut self) {
        let mut extensible_attributes = HashMap::new();
        for (k, v) in self.define_attributes.iter() {
            if v.extensible() {
                extensible_attributes.insert(k.clone(), v.clone());
            }
        }
        loop {
            let extensible = extensible_attributes.iter().any(|(_, v)| v.extensible());
            if !extensible {
                break;
            }
            extensible_attributes
                .values_mut()
                .filter(|v| v.extensible())
                .for_each(|v| {
                    let extend = v.extend_value().unwrap();
                    let other = self.define_attributes.get(extend).unwrap();
                    v.extend(other);
                })
        }
        self.define_attributes.iter_mut().for_each(|(k, v)| {
            if let Some(extended_value) = extensible_attributes.get(k) {
                *v = extended_value.clone();
            }
        });
    }

    fn substitute_defined_attributes(&mut self) {
        loop {
            let copy_defined_attributes = self.define_attributes.clone();
            let success = self
                .define_attributes
                .values_mut()
                .any(|attr| attr.substitute(&copy_defined_attributes));
            if !success {
                break;
            }
        }
    }

    fn substitute_add_attributes(&mut self) {
        for attribute in &mut self.add_attributes {
            attribute.substitute(&self.define_attributes);
        }
    }

    pub(crate) fn prepare(&mut self) {
        self.extend();
        self.substitute_defined_attributes();
        self.substitute_add_attributes();
        for attr in self.add_attributes.iter_mut() {
            attr.add_missing_material_attribute();
            attr.add_missing_transform_attribute();
        }
    }
}

#[cfg(test)]
mod test {

    use crate::transform::{Transform, Transformable};

    use super::*;

    fn default_yaml() -> String {
        include_str!("sample.yaml").to_string()
    }

    fn assert_value(value: &Value, expected: &str) -> Result<(), serde_yaml::Error> {
        let parsed_string = serde_yaml::to_string(value)?;
        let value_from_expected: Value = serde_yaml::from_str(expected)?;
        let parsed_expected_string = serde_yaml::to_string(&value_from_expected)?;
        assert_eq!(parsed_string.trim(), parsed_expected_string.trim());
        Ok(())
    }

    #[test]
    fn parse_from_yaml() {
        let yaml = default_yaml();
        let parser = Parser::from_yaml_without_preprocessing(&yaml);
        assert!(parser.is_some());
        let parser = parser.unwrap();
        assert_eq!(parser.add_attributes.len(), 22);
        assert_eq!(parser.define_attributes.len(), 8);
    }

    #[test]
    fn an_attribute_is_extensible_if_it_has_extend_key() {
        let yaml = default_yaml();
        let parser = Parser::from_yaml_without_preprocessing(&yaml).unwrap();

        let value = parser.define_attributes.get("blue-material").unwrap();
        assert!(value.extensible());
        assert_eq!(value.extend_value(), Some("white-material"));

        let key = "white-material".to_string();
        let value = parser.define_attributes.get(&key).unwrap();
        assert!(!value.extensible());
    }

    #[test]
    fn extend_defined_attribute() -> Result<(), serde_yaml::Error> {
        let yaml = default_yaml();
        let mut parser = Parser::from_yaml_without_preprocessing(&yaml).unwrap();

        let white_material = parser
            .define_attributes
            .get("white-material")
            .unwrap()
            .clone();

        let blue_material = parser.define_attributes.get_mut("blue-material").unwrap();
        blue_material.extend(&white_material);

        let blue_material = parser.define_attributes.get("blue-material").unwrap();
        let expected = "
define: blue-material
value:
  color: [0.537, 0.831, 0.914]
  diffuse: 0.7
  ambient: 0.1
  specular: 0.0
  reflective: 0.1
";
        assert_value(blue_material.raw_value(), expected)
    }

    #[test]
    fn extend_all_defined_attribute() {
        let yaml = default_yaml();
        let mut parser = Parser::from_yaml_without_preprocessing(&yaml).unwrap();
        parser.extend();
        for v in parser.define_attributes.values() {
            assert!(!v.extensible());
        }
    }

    #[test]
    fn substitute_define_attribute() -> Result<(), serde_yaml::Error> {
        let yaml = "
- define: standard-transform
  value:
  - [ translate, 1, -1, 1]
  - [ scale, 0.5, 0.5, 0.5]
- define: small-object
  value:
  - standard-transform
  - [ scale, 2, 2, 2]
        ";

        let mut parser = Parser::from_yaml_without_preprocessing(yaml).unwrap();
        parser.extend();
        parser.substitute_defined_attributes();

        let small_object = parser.define_attributes.get("small-object").unwrap();
        let expected = "
define: small-object
value:
- [ translate, 1, -1, 1]
- [ scale, 0.5, 0.5, 0.5]
- [ scale, 2, 2, 2]
        ";
        assert_value(small_object.raw_value(), expected)
    }

    #[test]
    fn substitute_add_attributes() -> Result<(), serde_yaml::Error> {
        let yaml = "
- define: white-material
  value:
    color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1
- define: blue-material
  extend: white-material
  value:
    color: [ 0.537, 0.831, 0.914 ]
- define: standard-transform
  value:
  - [ translate, 1, -1, 1 ]
  - [ scale, 0.5, 0.5, 0.5 ]
- define: large-object
  value:
    - standard-transform
    - [ scale, 3.5, 3.5, 3.5 ]
- add: cube
  material: blue-material
  transform:
    - large-object
    - [ translate, 8.5, 1.5, -0.5 ]
";

        let mut parser = Parser::from_yaml_without_preprocessing(yaml).unwrap();
        parser.extend();
        parser.substitute_defined_attributes();
        parser.substitute_add_attributes();

        let cube = &parser.add_attributes[0];
        let expected = "
add: cube
material: 
  color: [ 0.537, 0.831, 0.914 ]
  diffuse: 0.7
  ambient: 0.1
  specular: 0.0
  reflective: 0.1
transform:
  - [ translate, 1, -1, 1 ]
  - [ scale, 0.5, 0.5, 0.5 ]
  - [ scale, 3.5, 3.5, 3.5 ]
  - [ translate, 8.5, 1.5, -0.5 ]
        ";
        assert_value(&cube.value(), expected)
    }

    #[test]
    fn parse_camera_from_str() {
        let yaml = "
- add : camera
  width: 10
  height: 20
  field-of-view: 1.25
  from: [ 1, 2, 3 ]
  to: [ 4, 5, 6 ]
  up: [ 7, 8, 9 ]
";
        let objects = from_str(yaml).unwrap();
        assert!(objects[0].as_camera().is_some());
    }

    #[test]
    fn parse_point_light_from_str() {
        let yaml = "
- add : camera
  width: 10
  height: 20
  field-of-view: 1.25
  from: [ 1, 2, 3 ]
  to: [ 4, 5, 6 ]
  up: [ 7, 8, 9 ]
- add: light
  at: [50, 100, -50]
  intensity: [1, 2, 3]
";
        let objects = from_str(yaml).unwrap();
        assert!(objects[0].as_camera().is_some());
        assert!(objects[1].as_light().is_some());
    }

    #[test]
    fn parse_sphere_without_transform_full_material() {
        let yaml = "
- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
    diffuse: 0.2
    ambient: 0.0
    specular: 1.0
    shininess: 200
    reflective: 0.7
    transparency: 0.7
    refractive-index: 1.5
        ";
        let objects = from_str(yaml).unwrap();
        let shape = objects[0].as_shape();
        assert!(shape.is_some());
        let sphere = shape.unwrap();
        assert!(sphere.as_sphere().is_some());
    }

    #[test]
    fn parse_sphere_without_transform_missing_material() {
        let yaml = "
- add: sphere
  material:
    diffuse: 0.2
        ";
        let objects = from_str(yaml).unwrap();
        let shape = objects[0].as_shape();
        assert!(shape.is_some());
        let sphere = shape.unwrap();
        assert!(sphere.as_sphere().is_some());
    }

    #[test]
    fn parse_sphere_without_transform_and_material() {
        let yaml = "
- add: sphere
        ";
        let objects = from_str(yaml).unwrap();
        let shape = objects[0].as_shape();
        assert!(shape.is_some());
        let sphere = shape.unwrap();
        assert!(sphere.as_sphere().is_some());
    }

    #[test]
    fn parse_sphere_with_transformation() {
        let yaml = "
- add: sphere
  transform:
  - ['translate', 1.0, 3.0, 2.0]
  - ['scale', 4.0, 5.0, 6.0]
  - ['rotate-x', 1.5]
        ";
        let objects = from_str(yaml).unwrap();
        let shape = objects[0].as_shape();
        assert!(shape.is_some());
        let sphere = shape.unwrap();
        let inversed = sphere.inversed_transform();
        let expected = Transform::translation(1.0, 3.0, 2.0)
            .scale(4.0, 5.0, 6.0)
            .rotate_x(1.5)
            .inverse();
        assert_eq!(inversed, expected);
    }

    #[test]
    fn parse_sample_yaml_without_panic() {
        let yaml = default_yaml();
        let _ = from_str(&yaml).unwrap();
    }
}
