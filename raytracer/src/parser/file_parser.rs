use std::{collections::HashMap, hash::Hash};

use serde_yaml::Value;

fn get_value_inside_attributes(
    value: &mut Value,
    attributes: &HashMap<String, DefineAttribute>,
) -> Option<Value> {
    let s = value.as_str()?;
    let value_inside = attributes.get(s)?.value()?;
    Some(value_inside.clone())
}

fn substitute(value: &mut Value, attributes: &HashMap<String, DefineAttribute>) -> bool {
    let mut success: bool = false;
    match value {
        Value::Mapping(m) => {
            for (k, v) in m {
                let key_string = k.as_str();
                if key_string == Some("define") {
                    continue;
                }
                if let Some(value_inside) = get_value_inside_attributes(v, attributes) {
                    *v = value_inside;
                    success = true;
                } else {
                    substitute(v, attributes);
                }
            }
        }
        Value::Sequence(seq) => {
            let mut values = Vec::new();
            for v in seq {
                if let Some(value_inside) = get_value_inside_attributes(v, attributes) {
                    if let Some(arr) = value_inside.as_sequence() {
                        for v in arr {
                            values.push(v.clone());
                        }
                    } else {
                        values.push(value_inside);
                    }
                    success = true;
                } else {
                    substitute(v, attributes);
                    values.push(v.clone());
                }
            }
            *value = Value::Sequence(values);
        }
        _ => (),
    };
    success
}

#[derive(Debug, Hash, Clone)]
struct AddAttribute {
    value: Value,
}

impl AddAttribute {
    fn new(value: Value) -> AddAttribute {
        AddAttribute { value }
    }
}

#[derive(Debug, Hash, Clone)]
struct DefineAttribute {
    value: Value,
}

impl DefineAttribute {
    fn new(value: Value) -> DefineAttribute {
        DefineAttribute { value }
    }
    fn value(&self) -> Option<&Value> {
        self.value.as_mapping()?.get("value")
    }
    fn extensible(&self) -> bool {
        self.value
            .as_mapping()
            .map_or(false, |mapping| mapping.contains_key("extend"))
    }
    fn extend_value(&self) -> Option<&str> {
        self.value.as_mapping()?.get("extend")?.as_str()
    }
    fn extend(&mut self, other: &DefineAttribute) -> Option<()> {
        if other.extensible() {
            return None;
        }
        let map = self.value.as_mapping_mut()?;
        map.remove("extend");
        let value = map.get_mut("value")?.as_mapping_mut()?;
        let other_value = other.value.get("value")?.as_mapping()?;
        for (k, v) in other_value.iter() {
            if !value.contains_key(k) {
                value.insert(k.clone(), v.clone());
            }
        }
        Some(())
    }
}

#[derive(Debug, Default)]
struct Parser {
    add_attributes: Vec<AddAttribute>,
    define_attributes: HashMap<String, DefineAttribute>,
}

impl Parser {
    fn from_yaml(yaml: &str) -> Option<Parser> {
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
                .any(|attr| substitute(&mut attr.value, &copy_defined_attributes));
            if !success {
                break;
            }
        }
    }

    fn substitute_add_attributes(&mut self) {
        for attribute in &mut self.add_attributes {
            let mut value = attribute.value.clone();
            substitute(&mut value, &self.define_attributes);
            attribute.value = value;
        }
    }

    pub(crate) fn prepare(&mut self) {
        self.extend();
        self.substitute_defined_attributes();
        self.substitute_add_attributes();
    }
}

#[cfg(test)]
mod test {

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
        let parser = Parser::from_yaml(&yaml);
        assert!(parser.is_some());
        let parser = parser.unwrap();
        assert_eq!(parser.add_attributes.len(), 22);
        assert_eq!(parser.define_attributes.len(), 8);
    }

    #[test]
    fn an_attribute_is_extensible_if_it_has_extend_key() {
        let yaml = default_yaml();
        let parser = Parser::from_yaml(&yaml).unwrap();

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
        let mut parser = Parser::from_yaml(&yaml).unwrap();

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
        assert_value(&blue_material.value, expected)
    }

    #[test]
    fn extend_all_defined_attribute() {
        let yaml = default_yaml();
        let mut parser = Parser::from_yaml(&yaml).unwrap();
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

        let mut parser = Parser::from_yaml(yaml).unwrap();
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
        assert_value(&small_object.value, expected)
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

        let mut parser = Parser::from_yaml(yaml).unwrap();
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
        assert_value(&cube.value, expected)
    }
}
