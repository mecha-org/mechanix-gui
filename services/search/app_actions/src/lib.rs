use serde::Deserialize;

pub mod service;
mod utils;

pub use crate::service::AppActionsService;
pub use crate::service::AppActions;
#[derive(Debug, Deserialize, Clone)]
pub struct AppActionsConfig {
    pub enable_search: bool,
    pub index_dir: String,
    pub schema_dir: String,
    pub search_limit: usize,
    searchable_fields: Vec<String>,
}

impl Default for AppActionsConfig {
    fn default() -> Self {
        Self {
            enable_search: false,
            index_dir: "/usr/share/applications".to_string(),
            schema_dir: ".config/mxsearch/index/app_actions".to_string(),
            search_limit: 10,
            searchable_fields: vec![ "name".to_string() ],
        }
    }
}