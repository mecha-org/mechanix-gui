use anyhow::Result;
use tokio::task::JoinHandle;
use zbus::connection;
mod config;
mod interfaces;
use config::read_configs_yml;

use interfaces::{
    hw_buttons_notification_stream, BluetoothBusInterface, DisplayBusInterface,
    HostMetricsBusInterface, HwButtonInterface, PowerBusInterface, WirelessBusInterface,
};

use interfaces::{
    bluetooth_event_notification_stream, host_metrics_event_notification_stream,
    power_event_notification_stream, wireless_event_notification_stream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = match read_configs_yml() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error reading configs: {}", e);
            std::process::exit(1);
        }
    };

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let bluetooth_bus = BluetoothBusInterface {};
    let _bluetooth_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Bluetooth")?
        .serve_at("/org/mechanix/services/Bluetooth", bluetooth_bus)?
        .build()
        .await?;

    let _bluetooth_handle = tokio::spawn(async move {
        if let Err(e) =
            bluetooth_event_notification_stream(&bluetooth_bus, &_bluetooth_bus_connection).await
        {
            println!("Error in bluetooth notification stream: {}", e);
        }
    });

    handles.push(_bluetooth_handle);

    let wireless_bus = WirelessBusInterface {
        path: config.interfaces.network.device.clone(),
    };
    let _wireless_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Wireless")?
        .serve_at("/org/mechanix/services/Wireless", wireless_bus.clone())?
        .build()
        .await?;

    let wireless_handle = tokio::spawn(async move {
        if let Err(e) = wireless_event_notification_stream(
            config.interfaces.network.device.clone(),
            &wireless_bus,
            &_wireless_bus_connection,
        )
        .await
        {
            println!("Error in wireless notification stream: {}", e);
        }
    });

    handles.push(wireless_handle);

    let power_bus = PowerBusInterface {};
    let power_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Power")?
        .serve_at("/org/mechanix/services/Power", power_bus)?
        .build()
        .await?;

    let power_handle = tokio::spawn(async move {
        if let Err(e) = power_event_notification_stream(&power_bus, &power_bus_connection).await {
            println!("Error in power notification stream: {}", e)
        }
    });

    handles.push(power_handle);

    let display_bus = DisplayBusInterface {
        path: config.interfaces.display.device.clone(),
    };
    let _display_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Display")?
        .serve_at("/org/mechanix/services/Display", display_bus)?
        .build()
        .await?;

    let host_metrics_bus = HostMetricsBusInterface {};
    let host_metrics_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.HostMetrics")?
        .serve_at(
            "/org/mechanix/services/HostMetrics",
            host_metrics_bus.clone(),
        )?
        .build()
        .await?;

    let _host_metrics_handle = tokio::spawn(async move {
        if let Err(e) =
            host_metrics_event_notification_stream(&host_metrics_bus, &host_metrics_bus_connection)
                .await
        {
            println!("Error in host_metrics_handle notification stream: {}", e)
        }
    });

    let hw_button_bus = HwButtonInterface {};
    let _hw_button_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.HwButton")?
        .serve_at("/org/mechanix/services/HwButton", hw_button_bus)?
        .build()
        .await?;
    let power_button_path = config.interfaces.hw_buttons.power.path.clone();
    let home_button_path = config.interfaces.hw_buttons.home.path.clone();

    let _hw_button_handle = tokio::spawn(async move {
        if let Err(e) = hw_buttons_notification_stream(
            &hw_button_bus,
            &_hw_button_bus_connection,
            power_button_path,
            home_button_path,
        )
        .await
        {
            println!("Error in power btn notification stream: {}", e);
        }
    });

    handles.push(_hw_button_handle);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
