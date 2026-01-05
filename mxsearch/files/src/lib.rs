use serde::Deserialize;
use std::collections::HashSet;

mod error;
mod service;
mod utils;

pub use crate::service::FileSearchService;
pub use service::FileInfo;
#[derive(Debug, Deserialize, Clone)]
pub struct FilesConfig {
    pub enable_search: bool,
    pub index_dir: String,
    pub files_dir_to_watch: String,
    pub max_depth: usize,
    pub max_watchers: usize,
    pub search_limit: usize,
    pub target_memory_usage_in_bytes: usize,
    pub read_file_content_upto_in_kb: usize,
    pub searchable_fields: Vec<String>,
    pub allowed_extensions: HashSet<String>,
}
