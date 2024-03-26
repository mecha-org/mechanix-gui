use crate::gui::WirelessConnectedState;
use crate::modules::wireless::errors::{WirelessServiceError, WirelessServiceErrorCodes};
use crate::WirelessStatus;
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";

        let wireless_states = vec![
            WirelessStatus::NotFound,
            WirelessStatus::Off,
            WirelessStatus::On,
            WirelessStatus::Connected(WirelessConnectedState::Weak),
            WirelessStatus::Connected(WirelessConnectedState::Low),
            WirelessStatus::Connected(WirelessConnectedState::Good),
            WirelessStatus::Connected(WirelessConnectedState::Strong),
        ];

        let current_state = *wireless_states
            .get((Local::now().second() % 7) as usize)
            .unwrap();

        Ok(current_state)
    }
}
