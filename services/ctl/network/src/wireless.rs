use anyhow::{bail, Result};
use std::process::Command;
use tokio::task::JoinHandle;
use tracing::{error as trace_error, info, trace};
use wifi_ctrl::sta::{self, Broadcast, BroadcastReceiver, NetworkResult, ScanResult, SelectResult};

use crate::errors::{WirelessNetworkError, WirelessNetworkErrorCodes};

pub struct WirelessNetworkControl {
    pub path: String,
}

impl WirelessNetworkControl {
    pub fn new(path: &str) -> Self {
        trace!(task = "wireless network instance", "init");
        // Check if the path is valid
        WirelessNetworkControl {
            path: String::from(path),
        }
    }

    async fn setup_wifi(
        &self,
    ) -> Result<(sta::RequestClient, sta::BroadcastReceiver, JoinHandle<()>)> {
        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                println!("wireless network setup successful");
                info!(
                    task = "wireless_network_setup",
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

        setup.set_socket_path(self.path.clone());

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let runtime_handle = tokio::spawn(async move {
            if let Err(e) = runtime.run().await {
                trace_error!(task = "setup_wifi", "error: {}", e);
            }
        });

        Ok((requester, broadcast, runtime_handle))
    }

    pub async fn status(&self) -> bool {
        trace!(
            task = "wireless_network_status",
            "checking wireless network status"
        );

        let output = Command::new("nmcli")
            .arg("r")
            .output()
            .expect("failed to execute nmcli");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        if lines.len() > 1 {
            let wifi_status: Vec<&str> = lines[1].split_whitespace().collect();
            return wifi_status[1] == "enabled";
        }

        false
    }

    pub async fn enable(&self) -> Result<()> {
        trace!(
            task = "enable_wireless_network",
            "enabling wireless network"
        );
        let output = Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .expect("Failed to execute nmcli command");

        info!(
            task = "enable_wireless_network",
            "stdout: {:?}", output.stdout
        );

        if output.status.success() {
            info!(
                task = "enable_wireless_network",
                "wireless network is enabled"
            );
            Ok(())
        } else {
            trace_error!(
                task = "enable_wireless_network",
                "unable to enable wireless network"
            );
            bail!(WirelessNetworkError::new(
                WirelessNetworkErrorCodes::UnableToTurnOnWirelessNetwork,
                "unable to enable wireless network".to_string(),
            ))
        }
    }

    pub async fn disable(&self) -> Result<()> {
        trace!(
            task = "disable_wireless_network",
            "disabling wireless network"
        );
        let output = Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .output()
            .expect("Failed to execute nmcli command");

        info!(
            task = "disable_wireless_network",
            "stdout: {:?}", output.stdout
        );

        if output.status.success() {
            info!(
                task = "disable_wireless_network",
                "wireless network is disabled"
            );
            Ok(())
        } else {
            trace_error!(
                task = "disable_wireless_network",
                "unable to disable wireless network"
            );
            bail!(WirelessNetworkError::new(
                WirelessNetworkErrorCodes::UnableToTurnOffWirelessNetwork,
                "unable to disable wireless network".to_string(),
            ))
        }
    }

    pub async fn scan(&self) -> Result<Vec<ScanResult>> {
        trace!(task = "scan_wireless_network", "init");
        let (requester, broadcast, runtime_handle) = self.setup_wifi().await?;

        let wireless_network_list =
            match WirelessNetworkControl::wireless_network_list(&self, requester).await {
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
        runtime_handle.await.ok();
        drop(broadcast);

        Ok(wireless_network_list)
    }

    async fn wireless_network_list(
        &self,
        requester: sta::RequestClient,
    ) -> Result<Vec<ScanResult>> {
        trace!(task = "wireless_network_list", "requesting scan");
        let scan = requester.get_scan().await?;
        requester.shutdown().await?;
        Ok(scan.to_vec())
    }

    pub async fn known_network(&self) -> Result<Vec<NetworkResult>> {
        trace!(
            task = "get_known_wireless_networks",
            "starting wireless network connection"
        );

        let (requester, broadcast, _runtime_handle) = self.setup_wifi().await?;

        //use known_wireless_networks to get the list of all the known wireless network networks or else return an error with matching error code
        let wireless_network_list = match self.known_wireless_networks(requester).await {
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
        drop(broadcast);

        Ok(wireless_network_list)
    }

    async fn known_wireless_networks(
        &self,
        requester: sta::RequestClient,
    ) -> Result<Vec<NetworkResult>> {
        trace!(task = "known_wireless_networks", "requesting networks");
        let scan = requester.get_networks().await?;
        requester.shutdown().await?;
        Ok(scan)
    }

    // we need to write function that return the currnet wireless network name if it is connected to wireless network or else none, how we're going to do that is we use get_known_wireless_networks function to get the list of all the known wireless network networks and from that reult we can filter the list that has  "flags": "[CURRENT]" and return the ssid of that network or else return none
    pub async fn info(&self) -> Result<ScanResult> {

        let known_wifi_list = self.known_network().await?;
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

    pub async fn connect(&self, ssid: &str, psk: &str) -> Result<()> {
        trace!(
            task = "connect_wireless_network",
            "starting wireless network connection"
        );

        let (requester, broadcast, _runtime_handle) = self.setup_wifi().await?;

        let wireless_network_list = match self
            .connect_wireless_network_list(requester, ssid, psk)
            .await
        {
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

                bail!(e)
            }
        };

        Ok(wireless_network_list)
    }

    async fn connect_wireless_network_list(
        &self,
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
                // requester.select_network(network.network_id).await?;
                self.select_network(requester.clone(), network.network_id)
                    .await?;
                requester.clone().shutdown().await?;
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

        //save network configuration
        requester.save_config().await?;

        //select newly created network id or else return an error with matching error code
        let _ = match self.select_network(requester.clone(), network_id).await {
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
                println!("Error ---------: {}", e);
                //if unable to connect to network, remove the network
                self.remove_network(requester.clone(), network_id).await?;
                bail!(e)
            }
        };

        requester.shutdown().await?;
        Ok(())
    }

    // remove wireless network from known networks using network id
    pub async fn remove_wireless_network(&self, path: &str, network_id: usize) -> Result<()> {
        trace!(
            task = "remove_wireless_network",
            "removing wireless network"
        );

        let (requester, broadcast, _runtime_handle) =
            WirelessNetworkControl::new(path).setup_wifi().await?;

        //use remove_network to remove the wireless network or else return an error with matching error code
        let wireless_network_list = match self.remove_network(requester, network_id).await {
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

    async fn remove_network(&self, requester: sta::RequestClient, network_id: usize) -> Result<()> {
        trace!(task = "remove_network", "removing wireless network");
        requester.remove_network(network_id).await?;
        requester.shutdown().await?;
        Ok(())
    }

    async fn broadcast_listener(mut broadcast_receiver: BroadcastReceiver) -> Result<()> {
        trace!(task = "broadcast_listener", "listening for broadcasts");
        while let Ok(broadcast) = broadcast_receiver.recv().await {
            match broadcast {
                Broadcast::Disconnected => {
                    trace_error!(
                        task = "broadcast_listener",
                        "unable to get wireless network status"
                    );
                    bail!(WirelessNetworkError::new(
                        WirelessNetworkErrorCodes::Unknown,
                        format!("unable to connect to wifi network"),
                    ))
                }
                _ => info!("Broadcast: {:?}", broadcast),
            }
        }
        Ok(())
    }

    pub async fn select_wireless_network(&self, path: &str, network_id: usize) -> Result<()> {
        trace!(
            task = "select_wireless_network",
            "selecting wireless network"
        );

        let (requester, broadcast, _runtime_handle) =
            WirelessNetworkControl::new(path).setup_wifi().await?;

        //use remove_network to remove the wireless network or else return an error with matching error code
        let wireless_network_list = match self.select_network(requester, network_id).await {
            Ok(wireless_network_list) => {
                info!(
                    task = "select_wireless_network",
                    "wireless network list: {:?}", wireless_network_list
                );
                wireless_network_list
            }
            Err(e) => {
                trace_error!(
                    task = "select_wireless_network",
                    "unable to get wireless network status: {}",
                    e
                );
                println!("Error: {}", e);
                bail!(e);
            }
        };

        println!(
            "Wireless network selection output {:?}",
            wireless_network_list
        );
        Ok(wireless_network_list)
    }

    async fn select_network(&self, requester: sta::RequestClient, network_id: usize) -> Result<()> {
        trace!(task = "select_network", "selecting wireless network");
        let result = requester.select_network(network_id).await?;

        match result {
            SelectResult::Success => {
                info!("Successfully selected wireless network");
                requester.shutdown().await?;
                Ok(())
            }
            SelectResult::WrongPsk => {
                trace_error!("Wrong PSK for wireless network");

                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::WrongPsk,
                    format!("wrong PSK for wireless network")
                ))
            }

            SelectResult::NotFound => {
                trace_error!("Wireless network not found");

                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::NotFound,
                    format!("wireless network not found")
                ))
            }
            SelectResult::PendingSelect => {
                trace_error!("Select already pending for wireless network");
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::PendingSelect,
                    format!("select already pending for wireless network")
                ))
            }

            SelectResult::InvalidNetworkId => {
                trace_error!("Invalid network ID for wireless network");
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::InvalidNetworkId,
                    format!("invalid network ID for wireless network")
                ))
            }

            SelectResult::Timeout => {
                trace_error!("Select timeout for wireless network");
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::Timeout,
                    format!("select timeout for wireless network")
                ))
            }

            SelectResult::AlreadyConnected => {
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::AlreadyConnected,
                    format!("already connected to wireless network")
                ))
            }
        }
    }
}
