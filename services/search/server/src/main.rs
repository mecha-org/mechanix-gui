mod error;
mod server;
mod service;

use crate::error::ServerError;
use crate::server::{SERVED_AT, ServerInterface};
use anyhow::{Context, Result};
use app_actions::{AppActionsConfig, AppActionsService};
use apps::{AppSearchService, Apps as AppSearchConfig};
use sources::SourceSearchServiceConfig;
use sources::service::SourceSearchService;
use files::{FileSearchService, FilesConfig as FileSearchConfig};
use log::{debug, error, info};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
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
    pub source: SourceSearchServiceConfig,
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

    let mut app_search_service_opt: Option<AppSearchService> = None;
    if config.apps.enable_search {
        let mut app_search_service = match AppSearchService::new(&config.apps) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create app search service: {}", e);
                return Err(ServerError::FailedStartAppSearchService(e));
            }
        };
        match app_search_service.run().await {
            Ok(()) => debug!("AppSearchService started"),
            Err(e) => {
                error!("Failed to start AppSearchService: {}", e);
                return Err(ServerError::FailedStartAppSearchService(e));
            }
        }
        app_search_service_opt = Some(app_search_service);
    }

    let mut file_search_service_opt: Option<FileSearchService> = None;
    if config.files.enable_search {
        let mut file_search_service = match FileSearchService::new(&config.files) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create file search service: {}", e);
                return Err(ServerError::FailedStartFileSearchService(e));
            }
        };
        match file_search_service.run() {
            Ok(()) => debug!("FileSearchService started"),
            Err(e) => {
                error!("Failed to start FileSearchService: {}", e);
                return Err(ServerError::FailedStartFileSearchService(e));
            }
        }
        file_search_service_opt = Some(file_search_service);
    }

    let mut external_search_service_opt: Option<SourceSearchService> = None;
    if config.source.enable_search {
        let mut external_search_service = match SourceSearchService::new(&config.source) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create sources search service: {}", e);
                return Err(ServerError::FailedStartExternalSearchService(e));
            }
        };
        match external_search_service.run() {
            Ok(()) => debug!("ExternalSearchService started"),
            Err(e) => {
                error!("Failed to start ExternalSearchService: {}", e);
                return Err(ServerError::FailedStartExternalSearchService(e));
            }
        }
        external_search_service_opt = Some(external_search_service);
    }

    let mut app_action_service_opt: Option<AppActionsService> = None;
    if config.app_actions.enable_search {
        let mut app_action_service = match AppActionsService::new(&config.app_actions) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create app actions service: {}", e);
                return Err(ServerError::FailedStartAppActionsService(e));
            }
        };
        match app_action_service.run().await {
            Ok(()) => debug!("FileSearchService started"),
            Err(e) => {
                error!("Failed to start FileSearchService: {}", e);
                return Err(ServerError::FailedStartAppActionsService(e));
            }
        }
        app_action_service_opt = Some(app_action_service);
    }

    // Build and register the D-Bus server (blocking until shutdown)
    let config_server = ServerInterface {
        config: config.clone(),
        app_search_service: app_search_service_opt,
        file_search_service: file_search_service_opt,
        app_actions_service: app_action_service_opt,
        external_search_service: external_search_service_opt,
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
            // Build a proxy to call our own D-Bus method
            if let Ok(proxy) = build_proxy(&conn).await {
                match proxy.call_method("ShutdownAll", &()).await {
                    Ok(_) => debug!("ShutdownAll called successfully"),
                    Err(e) => error!("Failed to call ShutdownAll: {}", e),
                }
            }

            // Deregister the D-Bus object to drop ServerInterface and owned services
            if let Err(e) = conn
                .object_server()
                .remove::<ServerInterface, _>(SERVED_AT)
                .await
            {
                error!("Failed to deregister D-Bus object: {}", e);
            } else {
                debug!("D-Bus object deregistered: {}", SERVED_AT);
            }
            // Allow background tasks a moment to observe channel closure and flush
            sleep(Duration::from_millis(800)).await;
        }
        Err(e) => error!("Failed to receive SIGINT: {}", e),
    }
    Ok(())
}

async fn build_proxy(conn: &zbus::Connection) -> Result<zbus::Proxy<'_>> {
    let proxy = match zbus::ProxyBuilder::new(conn)
        .interface("org.mechanix.MxSearch")?
        .destination(CONNECTION_BUS_NAME)?
        .path(SERVED_AT)?
        .build()
        .await
    {
        Ok(proxy) => proxy,
        Err(e) => return Err(e.into()),
    };
    Ok(proxy)
}
