use crate::SearchConfig;
use anyhow::Result;
use app_actions::service::AppActions;
use apps::{AppInfo, AppSearchService};
use files::FileInfo;
use log::{debug, error, info, warn};
use std::sync::Arc;
use zbus::{dbus_interface, fdo::Error as ZbusError, SignalContext};

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
#[derive(Clone)]
pub struct ServerInterface {
    pub(crate) config: SearchConfig,
    pub app_search_service: Arc<AppSearchService>,
    pub file_search_service: Arc<files::FileSearchService>,
    pub app_actions_service: Arc<app_actions::AppActionsService>,
}

#[dbus_interface(name = "org.mechanix.MxSearch")]
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

    pub async fn search_applications(&self, search: &str) -> zbus::fdo::Result<Vec<AppInfo>> {
        info!("Search Apps: {}", search);
        if !self.config.apps.enable_search_apps {
            warn!("Search Apps is disabled");
            return Err(ZbusError::Failed("Search Apps is disabled".to_string()));
        }
        // At some point later: perform a search
        let results = match self
            .app_search_service
            .search(search, self.config.apps.search_limit)
        {
            Ok(results) => results,
            Err(err) => {
                error!("Error searching apps: {}", err);
                return Err(ZbusError::Failed("Error searching apps".to_string()));
            }
        };
        debug!("result: {:?}", results);
        Ok(results)
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
        info!("List applications");
        if !self.config.apps.enable_search_apps {
            warn!("Search Apps is disabled");
            return Err(ZbusError::Failed("Search Apps is disabled".to_string()));
        }
        // At some point later: perform a search
        let results = match self
            .app_search_service
            .list_applications(self.config.apps.search_limit)
        {
            Ok(results) => results,
            Err(err) => {
                error!("Error searching apps: {}", err);
                return Err(ZbusError::Failed("Error searching apps".to_string()));
            }
        };
        debug!("result: {:?}", results);
        Ok(results)
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
    pub async fn search_files(&self, search: &str) -> zbus::fdo::Result<Vec<FileInfo>> {
        info!("Search files: {}", search);
        if !self.config.files.enable_search_files {
            warn!("Search Files is disabled");
            return Err(ZbusError::Failed("Search Files is disabled".to_string()));
        }
        // At some point later: perform a search
        let results = match self
            .file_search_service
            .search(search, self.config.files.search_limit)
        {
            Ok(results) => results,
            Err(err) => {
                error!("Error searching files: {}", err);
                return Err(ZbusError::Failed("Error searching files".to_string()));
            }
        };
        debug!("result: {:?}", results);
        Ok(results)
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
        if !self.config.app_actions.enable_search {
            warn!("Search App Action is disabled");
            return Err(ZbusError::Failed(
                "Search App Action is disabled".to_string(),
            ));
        }
        // At some point later: perform a search
        let results = match self
            .app_actions_service
            .search(search, self.config.apps.search_limit)
        {
            Ok(results) => results,
            Err(err) => {
                error!("Error searching app actions: {}", err);
                return Err(ZbusError::Failed("Error searching app actions".to_string()));
            }
        };
        debug!("result: {:?}", results);
        Ok(results)
    }
}
