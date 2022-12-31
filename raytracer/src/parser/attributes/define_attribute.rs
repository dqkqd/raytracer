use serde_yaml::Value;

use crate::parser::yaml::DefineAttributes;

use super::util::substitute;

#[derive(Debug, Clone)]
pub(crate) struct DefineAttribute {
    value: Value,
}

#[allow(dead_code)]
impl DefineAttribute {
    pub fn new(value: Value) -> DefineAttribute {
        DefineAttribute { value }
    }

    pub fn value(&self) -> Option<&Value> {
        self.value.as_mapping()?.get("value")
    }

    pub fn raw_value(&self) -> &Value {
        &self.value
    }

    pub fn extensible(&self) -> bool {
        self.value
            .as_mapping()
            .map_or(false, |mapping| mapping.contains_key("extend"))
    }

    pub fn extend_value(&self) -> Option<&str> {
        self.value.as_mapping()?.get("extend")?.as_str()
    }

    pub fn extend(&mut self, other: &DefineAttribute) -> Option<()> {
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

    pub fn substitute(&mut self, attributes: &DefineAttributes) -> bool {
        substitute(&mut self.value, attributes)
    }
}
