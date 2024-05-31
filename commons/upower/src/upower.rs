use zbus::{proxy, zvariant::OwnedObjectPath, Result};
#[proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
pub trait UPower {
    /// EnumerateDevices method
    fn enumerate_devices(&self) -> Result<Vec<OwnedObjectPath>>;

    /// GetDisplayDevice method
    fn get_display_device(&self) -> Result<OwnedObjectPath>;
}
