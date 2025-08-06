//! NetworkManager helper service

use super::interfaces::NetworkManagerInterface;
use crate::errors::NetworkManagerError;
use crate::interfaces::wireless::{
    AccessPointEvent, EventType, NM80211ApFlags, NMState, WirelessNetworkInfo,
};
use crate::proxies::NetworkManagerProxy;
use anyhow::Result;
use futures::StreamExt;
use log::{debug, error, info};
use std::sync::mpsc;
use zbus::Connection;

/// A service wrapper for interacting with a NetworkManager implementation.
///
/// This generic struct provides high-level methods for managing WiFi connections,
/// such as enabling/disabling WiFi, listing available networks, and connecting to a network.
/// The implementation is generic over any type that implements `NetworkManagerInterface`.
#[derive(Clone)]
pub struct NetworkManagerService {
    proxy: NetworkManagerProxy<'static>,
}

impl NetworkManagerService {
    /// Creates a new `NetworkManagerService` with the given NetworkManager interface.
    ///
    /// # Arguments
    ///
    /// * `nm` - An object implementing the `NetworkManagerInterface` trait.
    /// Async constructor: handles connection and proxy creation internally.
    pub async fn new() -> Result<Self, NetworkManagerError> {
        let conn = Connection::system()
            .await
            .map_err(|e| NetworkManagerError::CreateNmProxyError(e.to_string()))?;
        let proxy = NetworkManagerProxy::new(&conn)
            .await
            .map_err(|e| NetworkManagerError::CreateNmProxyError(e.to_string()))?;
        info!("network manager proxy created");
        Ok(Self { proxy })
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
    pub async fn toggle_wireless(&self, enabled: bool) -> Result<(), NetworkManagerError> {
        self.proxy
            .toggle_wireless(enabled)
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
            .proxy
            .list_networks()
            .await
            .map_err(NetworkManagerError::from)?;

        // Convert raw access point data into user-friendly WirelessNetworkInfo structs.
        raw_access_points
            .into_iter()
            .map(|raw_ap| {
                let signal_strength = raw_ap.strength;
                // Determine the security type based on access point flags.
                let security = if raw_ap.nm80211_flags().contains(NM80211ApFlags::PRIVACY) {
                    "Protected".to_string()
                } else {
                    "Open".to_string()
                };

                Ok(WirelessNetworkInfo {
                    ssid: raw_ap.ssid,
                    signal_strength,
                    security,
                    hw_address: raw_ap.hw_address,
                    is_active: raw_ap.is_active,
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
        password: &Option<String>,
    ) -> Result<(), NetworkManagerError> {
        // Attempt to connect to the specified network using the NetworkManager interface.
        self.proxy
            .connect_to_network(ssid, password)
            .await
            .map_err(NetworkManagerError::from)?;
        Ok(())
    }

    // /// Get current Wireless status.
    // Pub async fn current_status(&self) -> Result<WifiStatus> {
    //     self.nm.current_status().await
    // }

    /// Subscribes to WiFi state change events from the NetworkManager.
    ///
    /// This method creates a channel to receive WiFi state updates and spawns a background
    /// task that listens for events from the NetworkManager. The events are forwarded
    /// through the returned receiver.
    ///
    /// # Returns
    ///
    /// Returns a `mpsc::Receiver<WifiState>` that will receive WiFi state change events.
    /// The channel has a buffer size of 32 messages.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio;
    /// use freedesktop_network_manager_client::service::NetworkManagerService;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let nm_service = NetworkManagerService::new();
    ///     let mut rx = nm_service.stream_device_events().await;
    ///
    ///     while let Some(state) = rx.recv().await {
    ///         println!("WiFi state changed: {:?}", state);
    ///     }
    /// }
    /// ```

    pub async fn connect_to_saved_network(&self, ssid: &str) -> Result<(), NetworkManagerError> {
        self.proxy
            .connect_to_saved_network(ssid)
            .await
            .map_err(NetworkManagerError::from)
    }
    pub async fn forget_saved_network(&self, ssid: &str) -> Result<(), NetworkManagerError> {
        self.proxy
            .forget_saved_network(ssid)
            .await
            .map_err(NetworkManagerError::from)
    }

    pub async fn disconnect_network(&self) -> Result<(), NetworkManagerError> {
        self.proxy
            .disconnect()
            .await
            .map_err(NetworkManagerError::from)
    }

    pub async fn stream_device_events(&self) -> mpsc::Receiver<NMState> {
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_device_events().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                // Blocking send (uses thread park/unpark internally)
                                if let Err(e) = sender.send(NMState::from(state)) {
                                    error!("failed to send device event to receiver: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device events: {}", e);
                    }
                }
            });
        });
        receiver
    }

    pub async fn stream_access_point_events(
        &self,
    ) -> mpsc::Receiver<Result<AccessPointEvent, NetworkManagerError>> {
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (mut access_point_added_stream, mut access_point_removed_stream) =
                    match proxy.stream_access_point_events().await {
                        Ok((added_stream, removed_stream)) => (added_stream, removed_stream),
                        Err(e) => {
                            error!("failed to stream to access point events: {}", e);
                            return;
                        }
                    };

                loop {
                    tokio::select! {
                        // Handle access point added events
                        may_be_ap_added = access_point_added_stream.next() => {
                            if let Some(ap_added) = may_be_ap_added {
                                let args = match ap_added.args() {
                                    Ok(args) => args,
                                    Err(e) => {
                                        error!("failed to get access point added args: {}", e);
                                        continue; // Skip this item
                                    }
                                };
                                let access_point_path = args.access_point.to_string();
                                info!("access point added: {}", access_point_path);

                                let raw_access_point_info = match proxy.get_access_point_info(&args.access_point).await {
                                    Ok(info) => info,
                                    Err(e) => {
                                        error!("failed to get access point info: {}", e);
                                        continue; // Skip this item
                                    }
                                };

                                let access_point_event_info = AccessPointEvent {
                                    access_point_path,
                                    event_type: EventType::Added,
                                    raw_access_point_info: Some(raw_access_point_info),
                                    ..Default::default()
                                };

                                // Forward object path to the channel
                                if sender.send(Ok(access_point_event_info)).is_err() {
                                    error!("failed to send access point added: receiver dropped");
                                    continue; // Receiver dropped
                                }
                            } else {
                                continue; // Stream ended
                            }
                        }

                        // Handle access point removed events
                        may_be_ap_removed = access_point_removed_stream.next() => {
                            if let Some(ap_removed) = may_be_ap_removed {
                                println!("ap removed");
                                let args = match ap_removed.args() {
                                    Ok(args) => args,
                                    Err(e) => {
                                        error!("failed to get access point removed args: {}", e);
                                        continue; // Skip this item
                                    }
                                };
                                let access_point_path = args.access_point.to_string();
                                debug!("access point removed: {}", access_point_path);

                                let access_point_event_info = AccessPointEvent {
                                    access_point_path,
                                    event_type: EventType::Removed,
                                    raw_access_point_info: None,
                                    ..Default::default()
                                };
                                println!("event to send back: {:?}", access_point_event_info);

                                // Forward object path to the channel
                                if sender.send(Ok(access_point_event_info)).is_err() {
                                    error!("failed to send access point added: receiver dropped");
                                    continue; // Receiver dropped
                                }
                            } else {
                                continue; // Stream ended
                            }
                        }
                    }
                }
            });
        });
        receiver
    }
    pub async fn stream_wireless_enabled_status(&self) -> mpsc::Receiver<bool> {
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_wireless_enabled_status().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                info!("state updated: {}", state);
                                // Blocking send (uses thread park/unpark internally)
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send device event to receiver: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device events: {}", e);
                    }
                }
            });
        });
        receiver
    }
    pub async fn stream_active_network_strength(&self) -> mpsc::Receiver<u8> {
        info!("service-action:: streaming active network strength");
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_wireless_network_strength().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                // Blocking send (uses thread park/unpark internally)
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send strength event to receiver: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device events: {}", e);
                    }
                }
            });
        });
        receiver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::wireless::RawAccessPointInfo;
    use crate::proxies::wireless::{AccessPointAddedStream, AccessPointRemovedStream};
    use crate::proxies::ProxyError;
    use anyhow::Result;
    use mockall::{mock, predicate::*};

    // TODO: Issue with return type of the method stream_device_events so i have written a mock trait
    // Err: cannot return reference to temporary value
    #[async_trait::async_trait]
    pub trait NetworkManagerInterfaceMock {
        async fn toggle_wireless(&self, enabled: bool) -> Result<(), ProxyError>;
        async fn list_networks(&self) -> Result<Vec<RawAccessPointInfo>, ProxyError>;
        async fn connect_to_network(
            &self,
            ssid: &str,
            password: Option<String>,
        ) -> Result<(String, String), ProxyError>;
        async fn disconnect(&self) -> Result<(), ProxyError>;
        async fn get_access_point_info(
            &self,
            object_path: &str,
        ) -> Result<RawAccessPointInfo, ProxyError>;
        async fn stream_access_point_events(
            &self,
        ) -> Result<(AccessPointAddedStream, AccessPointRemovedStream), ProxyError>;
    }

    mock! {
        pub NetworkManager {}

        impl Clone for NetworkManager {
            fn clone(&self) -> Self;
        }

        #[async_trait::async_trait]
        impl NetworkManagerInterfaceMock for NetworkManager {
            async fn toggle_wireless(&self, enabled: bool) -> Result<(), ProxyError>;
            async fn list_networks(&self) -> Result<Vec<RawAccessPointInfo>, ProxyError>;
            async fn connect_to_network(&self, ssid: &str, password: Option<String>) -> Result<(String, String), ProxyError>;
            async fn disconnect(&self) -> Result<(), ProxyError>;
            async fn get_access_point_info(&self, object_path: &str) -> Result<RawAccessPointInfo, ProxyError>;
            // async fn stream_device_events(&self) -> Result<PropertyStream<u32>, ProxyError>;
            async fn stream_access_point_events(&self) -> Result<(AccessPointAddedStream, AccessPointRemovedStream), ProxyError>;
        }
    }

    // Helper to make a dummy RawAccessPointInfo
    fn make_ap(ssid: &str, strength: u8, privacy: bool) -> RawAccessPointInfo {
        RawAccessPointInfo {
            ssid: ssid.to_string(),
            strength,
            hw_address: "00:11:22:33:44:55".to_string(),
            ..Default::default()
        }
    }
    #[tokio::test]
    async fn test_toggle_wireless_success() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_toggle_wireless()
            .times(1)
            .returning(|_| Ok(()));

        let service = NetworkManagerService::new().await.unwrap();
        assert!(service.toggle_wireless(true).await.is_ok());
    }

    #[tokio::test]
    async fn test_toggle_wireless_failure() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_toggle_wireless()
            .returning(|_| Err(ProxyError::DbusCallFailed("Failed to enable WiFi".into())));

        let service = NetworkManagerService::new().await.unwrap();
        assert!(service.toggle_wireless(false).await.is_err());
    }

    #[tokio::test]
    async fn test_list_networks_success() {
        let mut mock_nm = MockNetworkManager::new();
        let ap1 = make_ap("TestWifi", 80, true);
        let ap2 = make_ap("OpenNet", 60, false);

        mock_nm
            .expect_list_networks()
            .returning(move || Ok(vec![ap1.clone(), ap2.clone()]));

        let service = NetworkManagerService::new().await.unwrap();
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

        let service = NetworkManagerService::new().await.unwrap();
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

        let service = NetworkManagerService::new().await.unwrap();
        let result = service
            .connect_network("TestWifi", &Some("password123".to_string()))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connect_network_error() {
        let mut mock_nm = MockNetworkManager::new();
        mock_nm
            .expect_connect_to_network()
            .returning(|_, _| Err(ProxyError::DbusCallFailed("Failed to connect".into())));

        let service = NetworkManagerService::new().await.unwrap();
        let result = service
            .connect_network("TestWifi", &Some("wrongpass".to_string()))
            .await;
        assert!(result.is_err());
    }
}
