use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Debug, Default, Clone)]
pub struct FileMetadata {
    pub file_type: String,
    pub name: String,
    pub content: String,
    pub path: String,
    pub last_modified: String,
}

/// Reads a file and returns its metadata as `FileMetadata`.
///
/// The contents of the file are read up to `buffer_size_kb` kilobytes.
///
/// # Errors
///
/// Returns an `std::io::Error` if the file cannot be read.
pub fn get_file_metadata(
    path: &Path,
    buffer_size_kb: usize,
    allowed_extensions_to_index_content: &HashSet<String>,
) -> Result<FileMetadata, std::io::Error> {
    let mut file_info = FileMetadata::default();
    if let Some(ext) = path.extension() {
        if path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                // Get file name without extension
                file_info.name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                debug!(
                    "path.extension(): {:?}, path: {}",
                    path.extension(),
                    path.display()
                );
                file_info.file_type = ext.to_string_lossy().to_string();
                file_info.path = path.display().to_string();

                //Store last modified as a timestamp
                if let Ok(duration) = metadata.modified()?.duration_since(UNIX_EPOCH) {
                    let ts_str = duration.as_secs().to_string();
                    file_info.last_modified = ts_str;
                }
            }
        }
    }

    // Read file content if it's an allowed file type
    if allowed_extensions_to_index_content.contains(&file_info.file_type) {
        // Open and read the file up to 100KB
        let mut file = File::open(path)?;
        let mut buffer = vec![0u8; buffer_size_kb * 1024]; // 100KB buffer

        // Read up to 100KB
        let bytes_read = file.read(&mut buffer)?;

        // Optionally, trim unused buffer
        buffer.truncate(bytes_read);
        file_info.content = String::from_utf8_lossy(&buffer).to_string();
    }

    Ok(file_info)
}
