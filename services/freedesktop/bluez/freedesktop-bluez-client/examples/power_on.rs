//! Basic example: PowerOn the Bluetooth device using freedesktop-bluez-client

use freedesktop_bluez_client::service::BluetoothService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bluetooth_service = BluetoothService::new().await?;
    bluetooth_service.toggle_bluetooth(true).await?;
    Ok(())
}
