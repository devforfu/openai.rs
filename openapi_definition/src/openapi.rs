use std::path::Path;
use serde_yaml::{mapping::Mapping, Value};

pub type ResultYAML = Result<Value, Box<dyn std::error::Error>>;

pub fn generate_sanitized_spec(path: &impl AsRef<Path>) -> ResultYAML {
    let mut value = read_yaml(path)?;
    sanitize_spec_object(&mut value);
    Ok(value)
}

pub fn read_yaml(path: &impl AsRef<Path>) -> ResultYAML {
    let file = std::fs::File::open(path.as_ref())?;
    let yaml: Value = serde_yaml::from_reader(file)?;
    Ok(yaml)
}

pub fn sanitize_spec_object(obj: &mut Value) {
    match obj {
        Value::Mapping(mapping) => {
            filter_out_oai_keys(mapping);
            fix_default_null(mapping);
            fix_nested_array(mapping);
            for (_, value) in mapping.iter_mut() {
                sanitize_spec_object(value);
            }
        }
        Value::Sequence(seq) => {
            for item in seq.iter_mut() {
                sanitize_spec_object(item);
            }
        }
        _ => (),
    }
}

fn filter_out_oai_keys(mapping: &mut Mapping) {
    let oai_keys: Vec<Value> = mapping
        .keys()
        .filter(|key| {
            key.as_str()
                .map_or(false, |key_str| key_str.starts_with("oai"))
        })
        .cloned()
        .collect();

    for key in oai_keys {
        mapping.remove(key);
    }
}

fn fix_default_null(mapping: &mut Mapping) {
    let default_key = Value::String("default".to_string());
    let null = Value::Null;
    let obj_type = mapping.get("type").cloned();

    if let Some(default_value) = mapping.get_mut(&default_key) {
        if *default_value == null {
            if let Some(Value::String(s)) = obj_type {
                match s.as_str() {
                    "object" => *default_value = Value::Mapping(Mapping::new()),
                    "array" => *default_value = Value::Sequence(vec![]),
                    _ => (),
                }
            }
        }
    }
}

fn fix_nested_array(mapping: &mut Mapping) {
    match mapping.get("type") {
        Some(Value::String(s)) if s == "array" => {
            if let Some(Value::Mapping(items_map)) = mapping.get_mut("items") {
                if items_map.get("type") == Some(&Value::String("value".to_string())) {
                    *items_map = Mapping::new();
                }
            }
        }
        _ => (),
    }
}
