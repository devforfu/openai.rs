use log::{debug, info};
use reqwest::blocking::get;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

const OPENAPI_YAML_URL: &str =
    "https://raw.githubusercontent.com/openai/openai-openapi/master/openapi.yaml";

pub fn update_yaml(file_path: &impl AsRef<Path>) -> Result<(), Error> {
    debug!("requesting OpenAPI YAML");
    let mut response =
        get(OPENAPI_YAML_URL).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    debug!("reading response to a buffer");
    let mut buf = vec![];
    response.read_to_end(&mut buf)?;

    if file_path.as_ref().exists() {
        debug!("file exists; checking hash value...");
        let content = fs::read(file_path)?;
        let new_hash = compute_hash(&buf);
        let old_hash = compute_hash(&content);
        if new_hash == old_hash {
            info!(
                "file with the same signature exists: {}",
                file_path.as_ref().display()
            );
            return Ok(());
        }
    }

    debug!("writing file: {}", file_path.as_ref().display());
    fs::write(file_path, buf)?;

    Ok(())
}

fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
