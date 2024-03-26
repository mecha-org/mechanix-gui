
use tracing::info;
use zbus::{Connection, proxy , Result, zvariant::{ DeserializeDict, SerializeDict,Type},};

#[derive(DeserializeDict, SerializeDict, Debug, Type, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct DeviceInfo {
    pub os_name: String,
    pub os_version: String,
    pub serial_number: String,
    pub wifi_mac_address: String,
    pub ethernet_mac_address: String,
}

#[proxy(
    interface = "Mechanix.Services.DeviceInfo",
    default_service = "mechanix.services.deviceinfo",
    default_path = "/org/mechanix/services/deviceinfo"
)]
trait DisplayInfoBusInterface {
    async fn get_distro_info(&self) -> Result<DeviceInfo>;
}

 

impl DeviceInfo {
    pub async fn get_device_info_service() -> Result<DeviceInfo> {
        let connection = Connection::session().await?;
        let proxy = DisplayInfoBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_distro_info().await?;
        info!("Distro reply: {:?}", reply);
        Ok(reply)
    }
}
