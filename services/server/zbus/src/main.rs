use std::future::pending;
use anyhow::Result;
use zbus::connection;
mod interfaces;

use interfaces::{
    BluetoothBusInterface, DisplayBusInterface, PowerBusInterface, WirelessBusInterface,
};

#[tokio::main]
async fn main() -> Result<()> {
    let bluetooth_bus = BluetoothBusInterface {};
    let _bluetooth_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Bluetooth")?
        .serve_at("/org/mechanix/services/Bluetooth", bluetooth_bus)?
        .build()
        .await?;

    let wireless_bus = WirelessBusInterface {};
    let _wireless_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Wireless")?
        .serve_at("/org/mechanix/services/Wireless", wireless_bus)?
        .build()
        .await?;

    let power_bus = PowerBusInterface {};
    let _power_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Power")?
        .serve_at("/org/mechanix/services/Power", power_bus)?
        .build()
        .await?;

    let display_bus = DisplayBusInterface {};
    let _display_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Display")?
        .serve_at("/org/mechanix/services/Display", display_bus)?
        .build()
        .await?;

    pending::<()>().await;
    Ok(())
}
