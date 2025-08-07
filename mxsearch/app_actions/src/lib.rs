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
