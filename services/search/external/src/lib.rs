use serde::Deserialize;

pub mod service;
pub mod utils;
mod indexer;

#[derive(Debug, Deserialize, Clone)]
pub struct ExternalServiceConfig {
    pub enable_search: bool,
    pub index_dir: String,
    pub app_dir: String,
    pub search_limit: usize,
    pub target_memory_usage_in_bytes: usize,
    searchable_fields: Vec<String>,
}
