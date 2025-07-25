use crate::SearchConfig;
use anyhow::Result;
use apps::{AppInfo, AppSearchService};
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

    #[dbus_interface(signal)]
    async fn schema_key_changed(
        &self,
        ctxt: &SignalContext<'_>,
        schema: &str,
        key: &str,
        value: &str,
    ) -> Result<(), zbus::Error>;

    pub async fn search_apps(&self, search: &str) -> zbus::fdo::Result<Vec<AppInfo>> {
        info!("Search Apps: {}", search);
        if !self.config.apps.enable_search_apps {
            warn!("Search Apps is disabled");
            return Err(ZbusError::Failed("Search Apps is disabled".to_string()));
        }
        // At some point later: perform a search
        let results = match self.app_search_service.search(search, 10) {
            Ok(results) => results,
            Err(err) => {
                error!("Error searching apps: {}", err);
                return Err(ZbusError::Failed("Error searching apps".to_string()));
            }
        };
        debug!("result: {:?}", results);
        Ok(results)
    }
    pub async fn search_files(&self, search: &str) -> Result<String, ZbusError> {
        info!("Search Apps: {}", search);
        if !self.config.apps.enable_search_apps {
            warn!("Search Files is disabled");
            return Err(ZbusError::Failed("Search Files is disabled".to_string()));
        }
        Ok(search.to_string())
    }
}
