//! Handler to handle NetworkManager requests and events.

use super::interfaces::wireless::WirelessNetworkInfo;
use crate::errors::NetworkManagerError;
use crate::proxies::NetworkManagerProxy;
use crate::service::NetworkManagerService;
use anyhow::{bail, Result};
use log::error;
use tokio::{select, sync::mpsc};
use zbus::Connection;

/// Enum representing different requests that can be sent to the Client.
///
/// Each variant can carry data relevant to the request, such as parameters or reply channels.
#[derive(Debug)]
pub enum NetworkManagerRequest {
    /// Request to enable wireless device.
    EnableWirelessDevice {
        reply_to: mpsc::Sender<Result<(), NetworkManagerError>>,
    },
    /// Request to disable wireless device.
    DisableWirelessDevice {
        reply_to: mpsc::Sender<Result<(), NetworkManagerError>>,
    },
    /// Request to subscribe to device state change events.
    /// The response will be sent via the provided `reply_to` channel.
    GetDeviceStateChangeEvent {
        reply_to: mpsc::Sender<Result<String>>,
    },
    /// Request to get a list of available Wireless networks.
    /// The response will be sent via the provided `reply_to` channel.
    GetAvailableNetworks {
        reply_to: mpsc::Sender<Result<Vec<WirelessNetworkInfo>, NetworkManagerError>>,
    },
}

/// Handler for managing NetworkManager operations and requests.
///
/// This struct maintains a connection to the system D-Bus and processes incoming requests
/// through an async channel.
pub struct Client {
    /// D-Bus connection to the system bus.
    cn: Connection,
}

impl Client {
    /// Creates a new Client with a connection to the system D-Bus.
    pub async fn new() -> Self {
        Self {
            cn: Connection::system().await.unwrap(),
        }
    }

    /// Main event loop for handling incoming NetworkManager requests.
    ///
    /// # Arguments
    /// * `nm_request` - A receiver for incoming NetworkManagerRequest messages.
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok on normal operation, or an error if the proxy cannot be created.
    pub async fn run(
        &mut self,
        mut nm_request: mpsc::Receiver<NetworkManagerRequest>,
    ) -> Result<(), NetworkManagerError> {
        // 1. Create the NetworkManager proxy and service wrapper.
        let proxy = match NetworkManagerProxy::new(&self.cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create NetworkManager proxy: {}", e);
                return Err(NetworkManagerError::CreateNmProxyError(format!("{}", e)))
            },
        };
        let service = NetworkManagerService::new(proxy);

        // 2. Event loop: handle each incoming request.
        loop {
            select! {
                // Wait for the next request from the channel.
                msg = nm_request.recv() => {
                    // If a request was received, process it.
                    if let Some(request) = msg {
                        match request {
                            NetworkManagerRequest::EnableWirelessDevice { reply_to } => {
                                // Enable or disable WiFi using the service.
                                if let Err(e) = service.set_wifi(true).await {
                                    log::error!("failed to set wifi: {:?}", e);
                                }

                                if let Err(e) = reply_to.send(Ok(())).await {
                                    log::error!("failed to send response: {:?}", e);
                                }
                            }
                            NetworkManagerRequest::DisableWirelessDevice { reply_to } => {
                                // Enable or disable WiFi using the service.
                                if let Err(e) = service.set_wifi(false).await {
                                    log::error!("failed to set wifi: {:?}", e);
                                }

                                if let Err(e) = reply_to.send(Ok(())).await {
                                    log::error!("failed to send response: {:?}", e);
                                }
                            }
                            NetworkManagerRequest::GetDeviceStateChangeEvent { reply_to } => {
                                // Placeholder for subscribing to device state change events.
                                // Uncomment and implement event streaming as needed.
                                /*
                                match service.subscribe_events().await {
                                    Ok(mut stream) => {
                                        tokio::spawn(async move {
                                            while let Some(event) = stream.recv().await {
                                                // Convert WifiEvent to String (or your format)
                                                let msg = format!("{:?}", event);
                                                let _ = reply_to.send(Ok(msg)).await;
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        let _ = reply_to.send(Err(e)).await;
                                    }
                                }
                                */
                            }
                            NetworkManagerRequest::GetAvailableNetworks { reply_to } => {
                                // Use the service to list available wireless networks.
                                match service.list_networks().await {
                                    Ok(networks) => {
                                        if let Err(e) = reply_to.send(Ok(networks)).await {
                                            log::error!("failed to send response: {:?}", e);
                                        }
                                    }
                                    Err(e) => {
                                        if let Err(e) = reply_to.send(Err(e)).await {
                                            log::error!("failed to send response: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
