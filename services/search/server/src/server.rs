use crate::SearchConfig;
use anyhow::Result;
use app_actions::service::AppActions;
use apps::prelude::AppInfo;
use files::SearchResult;
use log::{debug, error, info, warn};
use sources::service::{SourceSearchResult, UpsertMetadata};
use std::sync::Arc;
use zbus::{fdo::Error as ZbusError, interface, SignalContext};

/// The D-Bus path where the ConfigServer interface is served
pub const SERVED_AT: &str = "/org/mechanix/MxSearch";

/// ConfigServerInterface struct for D-Bus interface.
///
/// This struct implements the D-Bus interface for the configuration server.
/// It provides methods for listing schemas, listing keys, describing keys,
/// getting settings, and setting settings. It also emits signals when
/// settings are changed.
///
/// The interface is served at the path defined by the SERVED_AT constant.
#[derive()]
pub struct ServerInterface {
    pub(crate) config: SearchConfig,
    pub app_search_service: Option<apps::prelude::AppSearchService>,
    pub file_search_service: Option<files::FileSearchService>,
    pub app_actions_service: Option<app_actions::AppActionsService>,
    pub external_search_service: Option<sources::service::SourceSearchService>,
}

#[interface(name = "org.mechanix.MxSearch")]
impl ServerInterface {
    /// Signal emitted when a setting is changed.
    ///
    /// This signal is emitted whenever a setting is changed through the set_setting method.
    /// Clients can listen for this signal to be notified of changes to settings they are
    /// interested in.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the setting that was changed
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the signal was emitted successfully
    /// * `Err(...)` if there was an error during emission

    pub async fn search_applications(
        &self,
        search: &str,
    ) -> zbus::fdo::Result<Vec<apps::prelude::AppInfo>> {
        info!("Search Apps: {}", search);

        if let Some(app_search_service) = &self.app_search_service {
            // At some point later: perform a search
            let results = match app_search_service.search(search, self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching apps: {}", err);
                    return Err(ZbusError::Failed("Error searching apps".to_string()));
                }
            };
            debug!("result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed("Search Apps is disabled".to_string()))
        }
    }

    /// Lists available applications.
    ///
    /// This function queries the application search service to retrieve a list of applications.
    /// It checks if the search functionality is enabled before proceeding.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of applications.
    ///
    /// # Returns
    ///
    /// A vector of `AppInfo` representing the available applications if successful.
    pub async fn list_applications(&self) -> zbus::fdo::Result<Vec<AppInfo>> {
        info!("List applications init");
        if let Some(app_search_service) = &self.app_search_service {
            // At some point later: perform a search
            let results = match app_search_service.list_applications(self.config.apps.search_limit)
            {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching apps: {}", err);
                    return Err(ZbusError::Failed("Error searching apps".to_string()));
                }
            };
            debug!("result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed("Search Apps is disabled".to_string()))
        }
    }

    /// Searches for files matching the given search string.
    ///
    /// This function queries the file search service to retrieve a list of files matching the search string.
    /// It checks if the search functionality is enabled before proceeding.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of files.
    ///
    /// # Returns
    ///
    /// A vector of `FileInfo` representing the matching files if successful.
    pub async fn search_files(&self, search: &str) -> zbus::fdo::Result<Vec<SearchResult>> {
        info!("Search files: {}", search);
        if let Some(file_search_service) = &self.file_search_service {
            // At some point later: perform a search
            let results = match file_search_service.search(search, self.config.files.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching files: {}", err);
                    return Err(ZbusError::Failed("Error searching files".to_string()));
                }
            };
            debug!("result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search Files service is disabled".to_string(),
            ))
        }
    }

    /// Searches for app actions matching the given search string.
    ///
    /// This function queries the app actions service to retrieve a list of app actions
    /// matching the search string. It checks if the search functionality is enabled before proceeding.
    ///
    /// # Arguments
    ///
    /// * `search` - A search string to query app actions.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of app actions.
    ///
    /// # Returns
    ///
    /// A vector of `AppActions` representing the matching app actions if successful.
    pub async fn search_app_actions(&self, search: &str) -> zbus::fdo::Result<Vec<AppActions>> {
        info!("Search app actions: {}", search);
        if let Some(app_actions_service) = &self.app_actions_service {
            // At some point later: perform a search
            let results = match app_actions_service.search(search, self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            debug!("result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search App Actions service is disabled".to_string(),
            ))
        }
    }

    pub async fn search_sources(&self, search: &str) -> zbus::fdo::Result<Vec<SourceSearchResult>> {
        info!("Search sources: {}", search);
        if let Some(service) = &self.external_search_service {
            // At some point later: perform a search
            let results = match service.search(search, self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search Sources service is disabled, enable it from settings.toml".to_string(),
            ))
        }
    }

    pub async fn upsert_sources(
        &mut self,
        metadata: Vec<UpsertMetadata>,
    ) -> zbus::fdo::Result<bool> {
        info!("upsert metadata: {:?}", metadata);
        if let Some(service) = self.external_search_service.as_mut() {
            // At some point later: perform a search
            let results = match service.upsert_metadata(metadata).await {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            debug!("result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search Sources service is disabled, enable it from settings.toml".to_string(),
            ))
        }
    }

    pub async fn delete_sources_by_ids(&mut self, ids: Vec<String>) -> zbus::fdo::Result<bool> {
        if let Some(service) = self.external_search_service.as_mut() {
            // At some point later: perform a search
            let results = match service.delete_by_ids(ids).await {
                Ok(results) => results,
                Err(err) => {
                    error!("Error deleting sources metadata by ids: {}", err);
                    return Err(ZbusError::Failed(
                        "Error deleting sources metadata".to_string(),
                    ));
                }
            };
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search Sources service is disabled, enable it from settings.toml".to_string(),
            ))
        }
    }

    pub async fn shutdown_all(&mut self) -> zbus::fdo::Result<()> {
        if let Some(svc) = &mut self.file_search_service {
            let _ = svc.shutdown().await;
        }
        if let Some(svc) = &mut self.app_search_service {
            let _ = svc.shutdown().await;
        }
        if let Some(svc) = &mut self.app_actions_service {
            let _ = svc.shutdown().await;
        }
        if let Some(svc) = &mut self.external_search_service {
            let _ = svc.shutdown().await;
        }
        Ok(())
    }
}
