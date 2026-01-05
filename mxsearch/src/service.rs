use crate::error::ServiceError;
use app_actions::AppActions;
use apps::AppInfo;
use files::FileInfo;
use log::{debug, info};
use zbus::{dbus_proxy, Connection};

#[derive(Clone)]
pub struct MxSearchService {
    proxy: MxSearchProxy<'static>,
}

#[dbus_proxy(
    interface = "org.mechanix.MxSearch",
    default_service = "org.mechanix.MxSearch",
    default_path = "/org/mechanix/MxSearch"
)]
pub trait MxSearch {
    async fn search_applications(&self, search: &str) -> zbus::fdo::Result<Vec<AppInfo>>;
    async fn list_applications(&self) -> zbus::fdo::Result<Vec<AppInfo>>;
    async fn search_files(&self, search: &str) -> zbus::fdo::Result<Vec<FileInfo>>;
    async fn search_app_actions(&self, search: &str) -> zbus::fdo::Result<Vec<AppActions>>;
}

impl MxSearchService {
    /// Creates a new `NetworkManagerService` with the given NetworkManager interface.
    ///
    /// # Arguments
    ///
    /// * `nm` - An object implementing the `NetworkManagerInterface` trait.
    /// Async constructor: handles connection and proxy creation internally.
    pub async fn new() -> anyhow::Result<Self, ServiceError> {
        let conn = Connection::session()
            .await
            .map_err(|e| ServiceError::CreateProxyError(e.to_string()))?;
        let proxy = MxSearchProxy::new(&conn)
            .await
            .map_err(|e| ServiceError::CreateProxyError(e.to_string()))?;
        info!("MxSearch proxy created");
        Ok(Self { proxy })
    }

    /// Get a setting from the server
    pub async fn list_applications(&self) -> Result<Vec<AppInfo>, anyhow::Error> {
        debug!("Connecting to D-Bus session for list_applications");
        let applications = self.proxy.list_applications().await?;
        Ok(applications)
    }
    pub async fn search_applications(&self, search: &str) -> Result<Vec<AppInfo>, anyhow::Error> {
        debug!("Connecting to D-Bus session for search_applications");
        let applications = self.proxy.search_applications(search).await?;
        Ok(applications)
    }

    pub async fn search_files(&self, search: &str) -> Result<Vec<FileInfo>, anyhow::Error> {
        debug!("Connecting to D-Bus session for search_files");
        let files = self.proxy.search_files(search).await?;
        Ok(files)
    }

    pub async fn search_app_actions(&self, search: &str) -> Result<Vec<AppActions>, anyhow::Error> {
        debug!("Connecting to D-Bus session for app actions");
        let app_actions = self.proxy.search_app_actions(search).await?;
        Ok(app_actions)
    }
}
