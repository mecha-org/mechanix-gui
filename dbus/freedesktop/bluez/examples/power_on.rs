//! Basic example: PowerOn the Bluetooth device using freedesktop-bluez-client

use bluez::service::BluetoothService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bluetooth_service = BluetoothService::new().await?;
    bluetooth_service.toggle_bluetooth(true).await?;
    Ok(())
}
