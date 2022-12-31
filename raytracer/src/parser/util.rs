use serde_yaml::Value;

use super::yaml::DefineAttributes;

pub(crate) fn default_transform() -> (Value, Value) {
    let value = serde_yaml::Sequence::new();
    let transform_key = Value::String("transform".to_string());
    let transform_value = Value::Sequence(value);
    (transform_key, transform_value)
}

pub(crate) fn default_material() -> (Value, Value) {
    let mut value = serde_yaml::Mapping::new();

    let all_keys_float = [
        ("diffuse", 0.9),
        ("ambient", 0.1),
        ("specular", 0.9),
        ("shininess", 200.0),
        ("reflective", 0.0),
        ("transparency", 0.0),
        ("refractive-index", 1.0),
    ];

    for (key, default_value) in all_keys_float {
        let value_num = Value::Number(serde_yaml::Number::from(default_value));
        let value_key = Value::String(key.to_string());
        value.insert(value_key, value_num);
    }

    let c = Value::Number(serde_yaml::Number::from(1.0));
    let value_color = Value::Sequence(vec![c.clone(), c.clone(), c]);
    let value_key = Value::String("color".to_string());
    value.insert(value_key, value_color);

    let material_key = Value::String("material".to_string());
    let material_value = Value::Mapping(value);
    (material_key, material_value)
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
