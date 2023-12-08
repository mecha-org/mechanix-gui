use crate::modules::wireless::errors::{WirelessServiceError, WirelessServiceErrorCodes};
use crate::{WirelessConnectedState, WirelessState};
use anyhow::{bail, Result};
use mecha_network_ctl::wireless_network::WirelessNetworkModule;
use tracing::{debug, error, info};

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessState> {
        let task = "get_wireless_status";

        //add mctl libs code here
        let is_wireless_on = WirelessNetworkModule::wireless_network_status();
        info!(task, "wireless status is {}", is_wireless_on);

        if !is_wireless_on {
            return Ok(WirelessState::Off);
        }

        let current_wireless_network = match WirelessNetworkModule.current_wireless_network().await
        {
            Ok(r) => r,
            Err(e) => {
                error!(
                    task,
                    "error while getting current connected wireless network {}", e
                );
                bail!(WirelessServiceError::new(
                    WirelessServiceErrorCodes::GetCurrentWirelessNetworkError,
                    format!(
                        "error while getting current connected wireless network {}",
                        e
                    ),
                    true
                ));
            }
        };
        debug!(
            task,
            "current wireless network is {:?}", current_wireless_network
        );

        if current_wireless_network.signal <= -80 {
            Ok(WirelessState::Connected(WirelessConnectedState::Low))
        } else if current_wireless_network.signal <= -60 {
            Ok(WirelessState::Connected(WirelessConnectedState::Weak))
        } else if current_wireless_network.signal <= -40 {
            Ok(WirelessState::Connected(WirelessConnectedState::Good))
        } else {
            Ok(WirelessState::Connected(WirelessConnectedState::Strong))
        }
    }
}
