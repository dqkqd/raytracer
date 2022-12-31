use serde_yaml::Value;

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
