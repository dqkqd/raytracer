use std::collections::HashSet;

use serde_yaml::Value;

use crate::parser::{
    objects::object::Object,
    util::{default_material, default_transform},
};

#[derive(Debug, Clone)]
pub(crate) struct AddAttribute {
    value: Value,
}

impl AddAttribute {
    pub fn new(value: Value) -> AddAttribute {
        AddAttribute { value }
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value
    }

    pub fn attribute_type(&self) -> &str {
        self.value["add"].as_str().unwrap()
    }

    pub fn parse(&self) -> Option<Object> {
        Object::from_attribute(self)
    }

    fn is_shape(&self) -> bool {
        matches!(self.attribute_type(), "sphere" | "plane" | "cube")
    }

    pub fn add_missing_transform_attribute(&mut self) -> Option<()> {
        if !self.is_shape() {
            return Some(());
        }

        let mapping = self.value.as_mapping_mut()?;
        if !mapping.contains_key("transform") {
            let (transform_key, transform_value) = default_transform();
            mapping.insert(transform_key, transform_value);
        }

        Some(())
    }

    pub fn add_missing_material_attribute(&mut self) -> Option<()> {
        if !self.is_shape() {
            return Some(());
        }

        let get_material_keys = || -> Option<HashSet<String>> {
            Some(
                self.value
                    .clone()
                    .as_mapping()?
                    .get("material")?
                    .as_mapping()?
                    .keys()
                    .into_iter()
                    .map(|k| k.as_str().unwrap().to_string())
                    .collect(),
            )
        };

        let keys = get_material_keys().unwrap_or_default();

        // build default material
        let (material_key, default_material) = default_material();

        let mapping = self.value.as_mapping_mut()?;
        if !mapping.contains_key("material") {
            mapping.insert(material_key, default_material);
        } else {
            let value = mapping.get_mut("material")?.as_mapping_mut()?;
            for (k, v) in default_material.as_mapping()? {
                let key = k.as_str()?.to_string();
                if !keys.contains(&key) {
                    value.insert(k.clone(), v.clone());
                }
            }
        }

        Some(())
    }
}
