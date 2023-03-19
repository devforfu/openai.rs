mod logger;
mod update;
mod openapi;

use std::fs;
use serde_yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::enable_debugging();

    let path = std::env::args().nth(1).unwrap_or("config/openapi.yaml".to_string());
    let sanitized = std::env::args().nth(2).unwrap_or("config/openapi_sanitized.yaml".to_string());

    update::update_yaml(&path)?;

    let spec = openapi::generate_sanitized_spec(&path)?;
    let file = fs::OpenOptions::new().create(true).write(true).open(sanitized)?;
    serde_yaml::to_writer(file, &spec)?;

    Ok(())
}
