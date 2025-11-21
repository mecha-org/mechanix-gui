mod error;
mod server;
mod service;

use crate::error::ServerError;
use crate::server::{ServerInterface, SERVED_AT};
use anyhow::{Context, Result};
use app_actions::{AppActionsConfig, AppActionsService};
use apps::{AppSearchService, Apps as AppSearchConfig};
use files::{FileSearchService, FilesConfig as FileSearchConfig};
use log::{debug, error, info};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use zbus::ConnectionBuilder;

const CONNECTION_BUS_NAME: &str = "org.mechanix.MxSearch";

#[derive(Debug, Deserialize, Clone)]
pub struct General {}
#[derive(Debug, Deserialize, Clone)]
pub struct SearchConfig {
    pub general: General,
    pub apps: AppSearchConfig,
    pub files: FileSearchConfig,
    pub app_actions: AppActionsConfig,
}
fn load_config<P: AsRef<Path>>(path: P) -> Result<SearchConfig> {
    info!("Loading config from {}", path.as_ref().display());
    let content = fs::read_to_string(path)?;
    let config: SearchConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Main function that sets up a file system watcher and a D-Bus server
///
/// # Returns
///
/// * `Ok(())` if the program ran successfully
/// * `Err(...)` if there was an error during execution
#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();
    let config = load_config("settings.toml")
        .context("Failed to load config")
        .unwrap();
    debug!("Loaded config: {:#?}", config);

    // Build the connection first
    let conn = match ConnectionBuilder::session() {
        Ok(builder) => match builder.name(CONNECTION_BUS_NAME) {
            Ok(named_builder) => match named_builder.build().await {
                Ok(conn) => conn,
                Err(e) => return Err(ServerError::FailedBuildConnection(e)),
            },
            Err(e) => return Err(ServerError::FailedBuildConnection(e)),
        },
        Err(e) => return Err(ServerError::FailedBuildConnection(e)),
    };

    debug!("D-Bus connection built");

    let mut app_search_service = match AppSearchService::new(&config.apps) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create app search service: {}", e);
            return Err(ServerError::FailedStartAppSearchService(e));
        }
    };

    let mut file_search_service = match FileSearchService::new(&config.files) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create file search service: {}", e);
            return Err(ServerError::FailedStartFileSearchService(e));
        }
    };

    let mut app_action_service = match AppActionsService::new(&config.app_actions) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create app actions service: {}", e);
            return Err(ServerError::FailedStartAppActionsService(e));
        }
    };

    if config.apps.enable_search {
        match app_search_service.run().await {
            Ok(()) => debug!("AppSearchService started"),
            Err(e) => {
                error!("Failed to start AppSearchService: {}", e);
                return Err(ServerError::FailedStartAppSearchService(e));
            }
        }
    }

    if config.files.enable_search {
        match file_search_service.run().await {
            Ok(()) => debug!("FileSearchService started"),
            Err(e) => {
                error!("Failed to start FileSearchService: {}", e);
                return Err(ServerError::FailedStartFileSearchService(e));
            }
        }
    }

    if config.app_actions.enable_search {
        match app_action_service.run().await {
            Ok(()) => debug!("FileSearchService started"),
            Err(e) => {
                error!("Failed to start FileSearchService: {}", e);
                return Err(ServerError::FailedStartAppActionsService(e));
            }
        }
    }
    // Build and register the D-Bus server (blocking until shutdown)
    let config_server = ServerInterface {
        config: config.clone(),
        app_search_service: app_search_service,
        file_search_service: file_search_service,
        app_actions_service: app_action_service,
    };

    debug!("D-Bus server registered at {}", SERVED_AT);

    if let Err(e) = conn.object_server().at(SERVED_AT, config_server).await {
        error!("Failed to start D-Bus server: {}", e);
        return Err(ServerError::FailedStartDBusServer(e));
    }
    // Wait for SIGINT (Ctrl+C)
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received SIGINT, shutting down");
        }
        Err(e) => error!("Failed to receive SIGINT: {}", e),
    }
    Ok(())
}
