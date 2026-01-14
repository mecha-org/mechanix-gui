use serde::Deserialize;
use std::collections::HashSet;

mod error;
mod service;
mod utils;

pub use crate::service::FileSearchService;
pub use service::SearchResult;
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
    pub content_index_extensions: HashSet<String>,
}

impl Default for FilesConfig {
    fn default() -> Self {
        Self {
            enable_search: false,
            index_dir: ".config/mxsearch/index/files".to_string(),
            files_dir_to_watch: "".to_string(),
            max_depth: 0,
            max_watchers: 0,
            search_limit: 0,
            target_memory_usage_in_bytes: 0,
            read_file_content_upto_in_kb: 0,
            searchable_fields: vec![],
            content_index_extensions: Default::default(),
        }
    }
}