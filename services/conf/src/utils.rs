use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::UNIX_EPOCH;
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


/// Reads a file and returns its metadata as `FileMetadata`.
///
/// # Errors
///
/// Returns an `std::io::Error` if the file cannot be read.
pub fn get_last_modified_timestamp(
    path: &Path,
) -> Result<u64, std::io::Error> {
    if let Some(_ext) = path.extension() {
        if path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                //Store last modified as a timestamp
                if let Ok(duration) = metadata.modified()?.duration_since(UNIX_EPOCH) {
                    return Ok(duration.as_secs())
                }
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "File not found"))
}
