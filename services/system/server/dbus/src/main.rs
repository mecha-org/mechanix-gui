use anyhow::Result;
use tokio::task::JoinHandle;
use zbus::connection;
mod config;
mod interfaces;
use config::read_configs_yml;

use interfaces::{
    hw_buttons_notification_stream, BluetoothBusInterface, DisplayBusInterface,
     HwButtonInterface, 
    // SecurityBusInterface,
};

use interfaces::
    bluetooth_event_notification_stream
;

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


    let display_bus = DisplayBusInterface {
        brightness_path: config.interfaces.display.brightness_path.clone(),
        backlight_path: config.interfaces.display.backlight_path.clone(),
    };
    let _display_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.Display")?
        .serve_at("/org/mechanix/services/Display", display_bus)?
        .build()
        .await?;

    let hw_button_bus = HwButtonInterface {};
    let _hw_button_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.HwButton")?
        .serve_at("/org/mechanix/services/HwButton", hw_button_bus)?
        .build()
        .await?;

    // let security_bus = SecurityBusInterface {};
    // let _security_bus_connection = connection::Builder::system()?
    //     .name("org.mechanix.services.Security")?
    //     .serve_at("/org/mechanix/services/Security", security_bus)?
    //     .build()
    //     .await?;

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
