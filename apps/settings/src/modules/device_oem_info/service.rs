
use tracing::info;
use zbus::{Connection, proxy , Result, zvariant::{ DeserializeDict, SerializeDict,Type},};

#[derive(DeserializeDict, SerializeDict, Debug, Type, Clone)]
#[zvariant(signature = "a{sv}")]
pub struct DistroInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_id: String,
    pub pretty_name: String,
    pub distro_codename: String,
    pub mac_address: String,
}

#[proxy(
    interface = "Mechanix.Services.DeviceInfo",
    default_service = "mechanix.services.deviceinfo",
    default_path = "/org/mechanix/services/deviceinfo"
)]
trait DisplayInfoBusInterface {
    async fn get_distro_info(&self) -> Result<DistroInfo>;
}

 
pub struct DeviceOEMInfo {}

impl DeviceOEMInfo {
    pub async fn get_device_oem_info_service() -> Result<DistroInfo> {
        let connection = Connection::session().await?;
        let proxy = DisplayInfoBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_distro_info().await?;
        info!("Distro reply: {:?}", reply);
        Ok(reply)
    }
}
