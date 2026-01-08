mod service;
mod utils;

use serde::Deserialize;

pub mod prelude {
    pub use crate::service::{AppInfo, AppSearchService};
    pub use crate::Apps;
}

/// App search service, watch dir for .desktop files and generate app info
/// Service recognize the events such as add, remove, update and services uses them to update the index
/// Service will generate the app info and store them in the index
/// Everytime the service is started, it will verify the checksum of .desktop files
/// If the checksum is different, it will update the index.
/// The app search service will return all the data it can from desktop files.
/// Index values as per https://specifications.freedesktop.org/desktop-entry-spec/latest
/// Endpoints:
/// /search_applications
/// Custom search across Name, Comment, Category, Additional Category, etc. giving proper weightage and scoring.
/// Should support wildcard search in Name.
/// /list_applications
/// /list_categories
/// Categorization to follow spec - https://specifications.freedesktop.org/menu-spec/latest/index.html#introduction

#[derive(Debug, Deserialize, Clone)]
pub struct Apps {
    pub enable_search: bool,
    pub index_dir: String,
    pub search_limit: usize,
    pub target_memory_usage_in_bytes: usize,
    searchable_fields: Vec<String>,
}
impl Default for Apps {
    fn default() -> Self {
        Self {
            enable_search: true,
            index_dir: ".config/mxsearch/index/apps".to_string(),
            search_limit: 5,
            target_memory_usage_in_bytes: 50_000_000,
            searchable_fields: vec!["name".to_string()],
        }
    }
}
