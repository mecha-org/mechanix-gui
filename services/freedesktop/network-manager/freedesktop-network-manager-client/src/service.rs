//! NetworkManager helper service

use super::interface::{
    NetworkManagerInterface,
};
use crate::error::NetworkManagerError;
use anyhow::Result;
use crate::interface::wireless::{NM80211ApFlags, WirelessNetworkInfo};

/// A service wrapper for interacting with a NetworkManager implementation.
///
/// This generic struct provides high-level methods for managing WiFi connections,
/// such as enabling/disabling WiFi, listing available networks, and connecting to a network.
/// The implementation is generic over any type that implements `NetworkManagerInterface`.
pub struct NetworkManagerService<T: NetworkManagerInterface> {
    /// The underlying NetworkManager interface implementation.
    nm: T,
}

impl<T: NetworkManagerInterface> NetworkManagerService<T> {
    /// Creates a new `NetworkManagerService` with the given NetworkManager interface.
    ///
    /// # Arguments
    ///
    /// * `nm` - An object implementing the `NetworkManagerInterface` trait.
    pub fn new(nm: T) -> Self {
        Self { nm }
    }

    /// Enables or disables wireless.
    ///
    /// # Arguments
    ///
    /// * `enabled` - If `true`, wireless will be enabled; if `false`, wireless will be disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying NetworkManager operation fails.
    pub async fn set_wifi(&self, enabled: bool) -> Result<(), NetworkManagerError> {
        self.nm
            .enable_wifi()
            .await
            .map_err(NetworkManagerError::from)
    }

    /// Lists available wireless networks.
    ///
    /// Queries the underlying NetworkManager for all visible access points,
    /// converts their raw information into `WirelessNetworkInfo` structs, and returns them.
    ///
    /// # Errors
    ///
    /// Returns a `NetworkManagerError::GetListOfNetworksError` if the operation fails.
    pub async fn list_networks(&self) -> Result<Vec<WirelessNetworkInfo>, NetworkManagerError> {
        // Fetch raw access point information from the NetworkManager.
        let raw_access_points = self
            .nm
            .list_networks()
            .await
            .map_err(NetworkManagerError::from)?;

        // Convert raw access point data into user-friendly WirelessNetworkInfo structs.
        raw_access_points
            .into_iter()
            .map(|raw_ap| {
                // Convert SSID bytes to a UTF-8 string.
                let ssid = String::from_utf8_lossy(&raw_ap.ssid).to_string();
                let signal_strength = raw_ap.strength;
                // Determine the security type based on access point flags.
                let security = if raw_ap
                    .nm80211_flags()
                    .contains(NM80211ApFlags::PRIVACY)
                {
                    "Protected".to_string()
                } else {
                    "Open".to_string()
                };

                Ok(WirelessNetworkInfo {
                    ssid,
                    signal_strength,
                    security,
                    hw_address: raw_ap.hw_address,
                })
            })
            .collect()
    }

    /// Attempts to connect to a Wireless network with the given SSID and optional password.
    ///
    /// # Arguments
    ///
    /// * `ssid` - The SSID of the Wireless network to connect to.
    /// * `password` - An optional password for the network (if required).
    ///
    /// # Errors
    ///
    /// Returns a `NetworkManagerError::ConnectToNewNetworkError` if the connection attempt fails.
    pub async fn connect_network(
        &self,
        ssid: &str,
        password: Option<String>,
    ) -> Result<(), NetworkManagerError> {
        // Attempt to connect to the specified network using the NetworkManager interface.
        self.nm
            .connect_to_network(ssid, password)
            .await
            .map_err(NetworkManagerError::from)?;
        Ok(())
    }

    // /// Get current Wireless status.
    // Pub async fn current_status(&self) -> Result<WifiStatus> {
    //     self.nm.current_status().await
    // }

    // /// Subscribe to Wireless events.
    // Pub async fn subscribe_events(&self) -> Result<BoxStream<'static, WifiEvent>> {
    //     self.nm.subscribe_events().await
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::wireless::RawAccessPointInfo;
    use crate::proxy::ProxyError;
    use anyhow::Result;
    use mockall::{mock, predicate::*};

    // 1. Mock the NetworkManagerInterface trait
    mock! {
        pub NetworkManager {}

        #[async_trait::async_trait]
        impl NetworkManagerInterface for NetworkManager {
            async fn enable_wifi(&self) -> Result<(), ProxyError>;
            async fn disable_wifi(&self) -> Result<(), ProxyError>;
            async fn list_networks(&self) -> Result<Vec<RawAccessPointInfo>, ProxyError>;
            async fn connect_to_network(&self, ssid: &str, password: Option<String>) -> Result<(String, String), ProxyError>;
            async fn disconnect(&self) -> Result<(), ProxyError>;
        }
    }

    // Helper to make a dummy RawAccessPointInfo
    fn make_ap(ssid: &[u8], strength: u8, privacy: bool) -> RawAccessPointInfo {
        RawAccessPointInfo {
            ssid: ssid.to_vec(),
            strength,
            hw_address: "00:11:22:33:44:55".to_string(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_set_wifi_success() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm.expect_disable_wifi().times(1).returning(|| Ok(()));

        let service = NetworkManagerService::new(mock_nm);
        assert!(service.set_wifi(true).await.is_ok());
    }

    #[tokio::test]
    async fn test_set_wifi_failure() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_enable_wifi()
            .returning(|| Err(ProxyError::DbusCallFailed("Failed to enable WiFi".into())));

        let service = NetworkManagerService::new(mock_nm);
        assert!(service.set_wifi(false).await.is_err());
    }

    #[tokio::test]
    async fn test_list_networks_success() {
        let mut mock_nm = MockNetworkManager::new();
        let ap1 = make_ap(b"TestWifi", 80, true);
        let ap2 = make_ap(b"OpenNet", 60, false);

        mock_nm
            .expect_list_networks()
            .returning(move || Ok(vec![ap1.clone(), ap2.clone()]));

        let service = NetworkManagerService::new(mock_nm);
        let networks = service.list_networks().await.unwrap();

        assert_eq!(networks.len(), 2);
        assert_eq!(networks[0].ssid, "TestWifi");
        assert_eq!(networks[0].security, "Protected");
        assert_eq!(networks[1].ssid, "OpenNet");
        assert_eq!(networks[1].security, "Open");
    }

    #[tokio::test]
    async fn test_list_networks_error() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_list_networks()
            .returning(|| Err(ProxyError::DbusCallFailed("Failed to list networks".into())));

        let service = NetworkManagerService::new(mock_nm);
        let result = service.list_networks().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_network_success() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_connect_to_network()
            .with(eq("TestWifi"), eq(Some("password123".to_string())))
            .returning(|_, _| Ok(("conn_path".into(), "active_path".into())));

        let service = NetworkManagerService::new(mock_nm);
        let result = service
            .connect_network("TestWifi", Some("password123".to_string()))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connect_network_error() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_connect_to_network()
            .returning(|_, _| Err(ProxyError::DbusCallFailed("Failed to connect".into())));

        let service = NetworkManagerService::new(mock_nm);
        let result = service
            .connect_network("TestWifi", Some("wrongpass".to_string()))
            .await;
        assert!(result.is_err());
    }
}
