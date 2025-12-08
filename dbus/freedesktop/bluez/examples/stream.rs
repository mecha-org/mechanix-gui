use futures::StreamExt;
use bluez::service::BluetoothService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bluez_service = BluetoothService::new().await?;
    let mut receiver = bluez_service.stream_bluetooth_device_status().await;

    let handler = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            println!("Bluetooth device status: {result:?}");
        }
    });

    handler.await.unwrap();
    Ok(())
}
