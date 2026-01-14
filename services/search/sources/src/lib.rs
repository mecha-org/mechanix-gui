use serde::Deserialize;

pub mod service;
pub mod utils;
mod indexer;

#[derive(Debug, Deserialize, Clone)]
pub struct SourceSearchServiceConfig {
    pub enable_search: bool,
    pub index_dir: String,
    pub app_dir: String,
    pub search_limit: usize,
    pub target_memory_usage_in_bytes: usize,
    searchable_fields: Vec<String>,
}

impl Default for SourceSearchServiceConfig {
    fn default() -> Self {
        Self {
            enable_search: false,
            index_dir: "".to_string(),
            app_dir: "".to_string(),
            search_limit: 0,
            target_memory_usage_in_bytes: 0,
            searchable_fields: vec![],
        }
    }
}