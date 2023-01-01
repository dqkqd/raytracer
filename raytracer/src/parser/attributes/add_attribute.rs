use serde_yaml::Value;

use crate::parser::{objects::object::Object, yaml::DefineAttributes};

use super::util::substitute;

#[derive(Debug, Clone)]
pub(crate) struct AddAttribute {
    value: Value,
    attribute_type: String,
}

#[allow(dead_code)]
impl AddAttribute {
    pub fn new(value: Value, attribute_type: String) -> AddAttribute {
        AddAttribute {
            value,
            attribute_type,
        }
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value
    }

    pub fn attribute_type(&self) -> &str {
        &self.attribute_type
    }

    pub fn parse(&self) -> Result<Object, serde_yaml::Error> {
        Object::from_attribute(self)
    }

    fn is_shape(&self) -> bool {
        matches!(self.attribute_type.as_str(), "sphere" | "plane" | "cube")
    }

    pub fn substitute(&mut self, attributes: &DefineAttributes) -> bool {
        substitute(&mut self.value, attributes)
    }
}
