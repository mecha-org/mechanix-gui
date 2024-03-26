use anyhow::{bail, Result};
use std::process::Command;
use tracing::{error as trace_error, info, trace};
use wifi_ctrl::sta::{self, NetworkResult, ScanResult};

use crate::errors::{WirelessNetworkError, WirelessNetworkErrorCodes};

pub struct WirelessNetworkControl;

impl WirelessNetworkControl {
    pub fn new() -> Self {
        trace!(task = "wireless network instance", "init");
        Self
    }

    pub async fn status(&self) -> bool {
        trace!(
            task = "wireless_network_status",
            "checking wireless network status"
        );
        let output = Command::new("ifconfig")
            .output()
            .expect("Failed to execute ifconfig command");

        info!(
            task = "wireless_network_status",
            "stdout: {:?}", output.stdout
        );
        let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

        // Check if the stdout contains "wlp2s0"
        if stdout.contains("wlp2s0") {
            info!(
                task = "wireless_network_status",
                "wireless network is up"
            );
            true
        } else {
            info!(
                task = "wireless_network_status",
                "wireless network is down"
            );
            false
        }
    }

    pub async fn scan(&self) -> Result<Vec<ScanResult>> {
        trace!(task = "scan_wireless_network", "init");
        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(
                    take = "wirelss_network_setup",
                    "wireless network setup successful"
                );
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "scan_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlp2s0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, wireless_network_list, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "scan_wireless_network", "error: {}", e);
                }
            },
            WirelessNetworkControl::wireless_network_list(requester),
            WirelessNetworkControl::broadcast_listener(broadcast),
        );

        //use wireless_network_list to get the list of all the wireless network networks or else return an error with matching error code
        let wireless_network_list = match wireless_network_list {
            Ok(wireless_network_list) => {
                info!(
                    task = "scan_wireless_network",
                    "wireless networks : {:?}", wireless_network_list
                );
                wireless_network_list
            }
            Err(e) => {
                trace_error!(
                    task = "scan_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };

        Ok(wireless_network_list)
    }

    async fn wireless_network_list(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
        trace!(task = "wireless_network_list", "requesting scan");
        let scan = requester.get_scan().await?;
        requester.shutdown().await?;
        Ok(scan.to_vec())
    }

    pub async fn known_network() -> Result<Vec<NetworkResult>> {
        trace!(
            task = "get_known_wireless_networks",
            "starting wireless network connection"
        );
        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(task = "wifi_setup", "wireless network setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "get_known_wireless_networks",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };
        let proposed_path = format!("/var/run/wpa_supplicant/wlp2s0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, known_wireless_networks, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "get_known_wireless_networks", "error: {}", e);
                }
            },
            WirelessNetworkControl::known_wireless_networks(requester),
            WirelessNetworkControl::broadcast_listener(broadcast),
        );

        //use known_wireless_networks to get the list of all the known wireless network networks or else return an error with matching error code
        let wireless_network_list = match known_wireless_networks {
            Ok(wireless_network_list) => {
                info!(
                    task = "get_known_wireless_networks",
                    "wireless networks: {:?}", wireless_network_list
                );
                wireless_network_list
            }
            Err(e) => {
                trace_error!(
                    task = "get_known_wireless_networks",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };

        Ok(wireless_network_list)
    }

    async fn known_wireless_networks(requester: sta::RequestClient) -> Result<Vec<NetworkResult>> {
        trace!(task = "known_wireless_networks", "requesting networks");
        let scan = requester.get_networks().await?;
        requester.shutdown().await?;
        Ok(scan)
    }

    // we need to write function that return the currnet wireless network name if it is connected to wireless network or else none, how we're going to do that is we use get_known_wireless_networks function to get the list of all the known wireless network networks and from that reult we can filter the list that has  "flags": "[CURRENT]" and return the ssid of that network or else return none
    pub async fn info(&self) -> Result<ScanResult> {
        let known_wifi_list = WirelessNetworkControl::known_network().await?;
        let current_wifi = known_wifi_list.iter().find(|&x| x.flags == "[CURRENT]");

        //take ssid for current wireless network and find that in scan_networks list and return that network or else return an error with matching error code
        let scan_wifi_list = WirelessNetworkControl::scan(&self).await?;
        let current_wifi = current_wifi
            .map(|x| {
                scan_wifi_list
                    .iter()
                    .find(|&y| y.name == x.ssid)
                    .map(|x| x.clone())
            })
            .flatten();

        match current_wifi {
            Some(current_wifi) => Ok(current_wifi.clone()),
            None => {
                trace_error!(
                    task = "currnet_wifi",
                    "unable to get current wireless network"
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get current wireless network"),
                ))
            }
        }
    }

    pub async fn connect(&self,ssid: &str, psk: &str) -> Result<()> {
        trace!(
            task = "connect_wireless_network",
            "starting wireless network connection"
        );

        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(task = "wifi_setup", "wireless network setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlp2s0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, connect_wireless_network_list, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "connect_wireless_network", "error: {}", e);
                }
            },
            WirelessNetworkControl::connect_wireless_network_list(requester, &ssid, &psk),
            WirelessNetworkControl::broadcast_listener(broadcast),
        );

        let wireless_network_list = match connect_wireless_network_list {
            Ok(wireless_network_list) => {
                info!(
                    task = "connect_wireless_network",
                    "wireless networks : {:?}", wireless_network_list
                );
                wireless_network_list
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork,
                    format!("unable to connect to wireless network {}", e),
                ))
            }
        };

        Ok(wireless_network_list)
    }

    async fn connect_wireless_network_list(
        requester: sta::RequestClient,
        ssid: &str,
        psk: &str,
    ) -> Result<()> {
        trace!(
            task = "connect_wireless_network_list",
            "requesting networks"
        );
        //handle networks or else return an error with matching error code
        let networks = match requester.get_networks().await {
            Ok(networks) => {
                info!(
                    task = "connect_wireless_network_list",
                    "networks: {:?}", networks
                );
                networks
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network_list",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork,
                    format!("unable to connect to wireless network {}", e),
                ))
            }
        };

        //if ssid is in known networks, use that network id to connect else create new network id
        for network in networks {
            if network.ssid == ssid {
                info!("network id: {}", network.network_id);
                requester.select_network(network.network_id).await?;
                requester.shutdown().await?;
                return Ok(());
            }
        }

        //if ssid is not in known networks, create new network id and connect to it or else return an error with matching error code
        let network_id = match requester.add_network().await {
            Ok(network_id) => {
                info!(
                    task = "connect_wireless_network_list",
                    "network id: {}", network_id
                );
                network_id
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network_list",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork,
                    format!("unable to connect to wireless network {}", e),
                ))
            }
        };

        //set network ssid
        requester
            .set_network_ssid(network_id, ssid.to_string())
            .await?;

        //set network psk
        requester
            .set_network_psk(network_id, psk.to_string())
            .await?;

        //select newly created network id or else return an error with matching error code
        let _ = match requester.select_network(network_id).await {
            Ok(_) => {
                info!(
                    task = "connect_wireless_network_list",
                    "connect to selected network"
                );
                ()
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network_list",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork,
                    format!("unable to connect to wireless network {}", e),
                ))
            }
        };

        requester.shutdown().await?;
        Ok(())
    }

    // remove wireless network from known networks using network id
    pub async fn remove_wireless_network(network_id: usize) -> Result<()> {
        trace!(
            task = "remove_wireless_network",
            "removing wireless network"
        );

        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(
                    task = "remove_wireless_network",
                    "wireless network setup successful"
                );
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "remove_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus,
                    format!("unable to get wireless network status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlp2s0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, remove_network, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "remove_wireless_network", "error: {}", e);
                }
            },
            WirelessNetworkControl::remove_network(requester, network_id),
            WirelessNetworkControl::broadcast_listener(broadcast),
        );

        //use remove_network to remove the wireless network or else return an error with matching error code
        let wireless_network_list = match remove_network {
            Ok(wireless_network_list) => {
                info!(
                    task = "remove_wireless_network",
                    "wireless network list: {:?}", wireless_network_list
                );
                wireless_network_list
            }
            Err(e) => {
                trace_error!(
                    task = "remove_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::UnableToRemoveWirelessNetwork,
                    format!("unable to remove wireless network {}", e),
                ))
            }
        };
        Ok(wireless_network_list)
    }

    async fn remove_network(requester: sta::RequestClient, network_id: usize) -> Result<()> {
        trace!(task = "remove_network", "removing wireless network");
        requester.remove_network(network_id).await?;
        requester.shutdown().await?;
        Ok(())
    }

    async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result<()> {
        trace!(task = "broadcast_listener", "listening for broadcasts");
        while let Ok(broadcast) = broadcast_receiver.recv().await {
            info!("Broadcast: {:?}", broadcast);
        }
        Ok(())
    }
}
