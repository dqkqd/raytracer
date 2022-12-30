use std::{collections::HashMap, hash::Hash};

use serde_yaml::Value;

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
    fn extensible(&self) -> bool {
        self.value
            .as_mapping()
            .map_or(false, |mapping| mapping.contains_key("extend"))
    }
    fn extend_value(&self) -> Option<&str> {
        self.value.as_mapping()?.get("extend")?.as_str()
    }
    fn extend(&mut self, other: &DefineAttribute) -> Option<()> {
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
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_yaml() -> String {
        include_str!("sample.yaml").to_string()
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
    fn extend_defined_attribute() {
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
        assert!(!blue_material.extensible());

        let mapping = blue_material.value.as_mapping().unwrap();
        let value = &mapping["value"];
        assert_eq!(value.as_mapping().unwrap().len(), 5);
        assert_eq!(value["diffuse"].as_f64(), Some(0.7));
        assert_eq!(value["ambient"].as_f64(), Some(0.1));
        assert_eq!(value["specular"].as_f64(), Some(0.0));
        assert_eq!(value["reflective"].as_f64(), Some(0.1));
        assert_eq!(value["color"][0].as_f64(), Some(0.537));
        assert_eq!(value["color"][1].as_f64(), Some(0.831));
        assert_eq!(value["color"][2].as_f64(), Some(0.914));
    }
}
