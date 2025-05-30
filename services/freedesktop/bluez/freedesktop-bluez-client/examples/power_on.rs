//! Basic example: PowerOn the Bluetooth device using freedesktop-bluez-client

use tokio::sync::{mpsc, oneshot};
use freedesktop_bluez_client::service::BluetoothService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bluetooth_service = BluetoothService::new().await?;
    bluetooth_service.set_powered_on().await?;
    Ok(())
}
