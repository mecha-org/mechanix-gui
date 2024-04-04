use anyhow::Result;
use std::{fs::File, future::pending, io::BufReader};
use zbus::connection;
mod config;
mod interfaces;
use config::BaseConfig;

use interfaces::{
    BluetoothBusInterface, DisplayBusInterface, HostMetricsBusInterface, PowerBusInterface,
    WirelessBusInterface,
};

#[tokio::main]
async fn main() -> Result<()> {
    
    let profile_file = File::open("./services_config.yml").expect("Failed to open config file");
    let reader = BufReader::new(profile_file);

    let config: BaseConfig = serde_yaml::from_reader(reader).expect("unable to rad yaml file");

    let bluetooth_bus = BluetoothBusInterface {};
    let _bluetooth_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Bluetooth")?
        .serve_at("/org/mechanix/services/Bluetooth", bluetooth_bus)?
        .build()
        .await?;

    let wireless_bus = WirelessBusInterface {
        path: config.interfaces.network.device.clone(),
    };
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

    let display_bus = DisplayBusInterface {
        path: config.interfaces.display.device.clone(),
    };
    let _display_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Display")?
        .serve_at("/org/mechanix/services/Display", display_bus)?
        .build()
        .await?;

    let host_metrics_bus = HostMetricsBusInterface {};
    let _host_metrics_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.HostMetrics")?
        .serve_at("/org/mechanix/services/HostMetrics", host_metrics_bus)?
        .build()
        .await?;

    pending::<()>().await;
    Ok(())
}
