use anyhow::Result;
use futures::StreamExt;
use upower::{device::DeviceProxy, upower::UPowerProxy, DeviceType, WarningLevel};
use zbus::{Connection, Proxy};

pub struct UpowerHandler {}
impl UpowerHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(mut self) {
        let _ = handle_upower_prop_update().await;
    }
}

async fn handle_upower_prop_update() -> Result<bool> {
    let connection = Connection::system().await?;
    let upower_p = UPowerProxy::builder(&connection).build().await?;
    let devices = upower_p.enumerate_devices().await?;
    let mut battery = None;
    for device_path in devices {
        let device = DeviceProxy::builder(&connection)
            .path(device_path)?
            .build()
            .await?;
        if device.type_().await? == DeviceType::Battery.into() {
            battery = Some(device);
            break;
        }
    }

    if battery.is_none() {
        return Ok(false);
    }

    let mut stream = battery.unwrap().receive_warning_level_changed().await;
    while let Some(msg) = stream.next().await {
        let warning_level = msg.get().await.unwrap();
        let warning = WarningLevel::from(warning_level);
        println!("warning {:?}", warning);
    }

    Ok(true)
}
