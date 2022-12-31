use serde_yaml::Value;

use crate::parser::yaml::DefineAttributes;

pub(crate) fn default_transform() -> (Value, Value) {
    let value = serde_yaml::Sequence::new();
    let transform_key = Value::String("transform".to_string());
    let transform_value = Value::Sequence(value);
    (transform_key, transform_value)
}

fn get_value_inside_attributes(value: &mut Value, attributes: &DefineAttributes) -> Option<Value> {
    let s = value.as_str()?;
    let value_inside = attributes.get(s)?.value()?;
    Some(value_inside.clone())
}

pub(crate) fn substitute(value: &mut Value, attributes: &DefineAttributes) -> bool {
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
