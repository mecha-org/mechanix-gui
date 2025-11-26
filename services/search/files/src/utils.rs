use crate::service::SearchResult;
use crate::FileSearchService;
use log::{debug, error};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use tantivy::schema::{Field, Schema, Value, STORED, STRING, TEXT};
use tantivy::TantivyDocument;

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
pub fn get_file_metadata(path: &Path) -> Result<FileMetadata, std::io::Error> {
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
    Ok(file_info)
}

pub fn read_file_content(path: &Path, buffer_size_kb: usize) -> Result<String, std::io::Error> {
    // Read file content if it's an allowed file type

    // Open and read the file up to 100KB
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; buffer_size_kb * 1024]; // 100KB buffer

    // Read up to 100KB
    let bytes_read = file.read(&mut buffer)?;

    // Optionally, trim unused buffer
    buffer.truncate(bytes_read);
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

#[derive(Debug, Clone)]
pub struct SchemaFields {
    pub file_type: Field,
    pub name: Field,
    pub content: Field,
    pub path: Field,
    pub last_modified: Field,
}
pub struct SchemaBundle {
    pub schema: Schema,
    pub fields: SchemaFields,
}

impl FileSearchService {
    // Build schema and pre-resolve field handles in one place
    pub(crate) fn build_schema() -> anyhow::Result<SchemaBundle> {
        let mut b = Schema::builder();
        // retrievable & searchable
        b.add_text_field("file_type", STRING | STORED);
        b.add_text_field("name", STRING | STORED);
        b.add_text_field("content", TEXT);
        b.add_text_field("path", STRING | STORED);
        b.add_u64_field("last_modified", STORED);

        let schema = b.build();

        // Reuse the existing helper to resolve the fields we care about in results
        let fields = match Self::build_fields(&schema) {
            Ok(fields) => fields,
            Err(e) => {
                error!("Failed to build fields: {}", e);
                return Err(e);
            }
        };

        Ok(SchemaBundle { schema, fields })
    }
    pub(crate) fn build_fields(schema: &Schema) -> anyhow::Result<SchemaFields> {
        Ok(SchemaFields {
            file_type: schema.get_field("file_type")?,
            name: schema.get_field("name")?,
            content: schema.get_field("content")?,
            path: schema.get_field("path")?,
            last_modified: schema.get_field("last_modified")?,
        })
    }

    // Map a tantivy document into our result struct using pre-resolved fields.
    pub(crate) fn map_doc(&self, doc: &TantivyDocument) -> SearchResult {
        let f = &self.schema_fields;
        SearchResult {
            file_type: get_text(doc, f.file_type).unwrap_or_default(),
            name: get_text(doc, f.name).unwrap_or_default(),
            path: get_text(doc, f.path).unwrap_or_default(),
            score: 0.0,
        }
    }
}

fn get_text(doc: &TantivyDocument, f: Field) -> Option<String> {
    doc.get_all(f)
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .next() // if you only expect single-valued fields
}
