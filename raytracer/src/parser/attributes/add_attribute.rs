use serde_yaml::Value;

use crate::parser::{objects::object::Object, yaml::DefineAttributes};

use super::util::{default_transform, substitute};

#[derive(Debug, Clone)]
pub(crate) struct AddAttribute {
    value: Value,
}

#[allow(dead_code)]
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

    pub fn substitute(&mut self, attributes: &DefineAttributes) -> bool {
        substitute(&mut self.value, attributes)
    }
}
