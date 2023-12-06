use crate::WirelessState;
use anyhow::Result;

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessState> {
        //add mctl libs code here

        Ok(WirelessState::On)
    }
}
