use serde_yaml::Value;

use crate::parser::{objects::object::Object, yaml::DefineAttributes};

use super::util::substitute;

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

    pub fn substitute(&mut self, attributes: &DefineAttributes) -> bool {
        substitute(&mut self.value, attributes)
    }
}
