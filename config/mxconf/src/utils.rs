use std::fs::File;
use std::io::Read;
use anyhow::Context;
use log::info;

/// Read and parse a TOML file, converting it to JSON bytes
///
/// # Arguments
///
/// * `path` - The path to the TOML file
///
/// # Returns
///
/// * `Ok(Vec<u8>)` containing the JSON bytes if successful
/// * `Err(...)` if there was an error during reading or conversion
pub fn read_application_schema(path: &str) -> anyhow::Result<String> {
    info!("Reading application schema file: {}", path);
    let mut file = File::open(path).with_context(|| format!("Unable to open file: {}", path))?;
    let mut contents_str = String::new();
    file.read_to_string(&mut contents_str)
        .with_context(|| format!("Unable to read file: {}", path))?;
    Ok(contents_str)
}
