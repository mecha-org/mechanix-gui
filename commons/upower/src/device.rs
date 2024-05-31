use serde::{Deserialize, Serialize};
use zbus::{proxy, Result};

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
)]
pub trait Device {
    /// WarningLevel property
    #[zbus(property)]
    fn warning_level(&self) -> Result<u32>;

    /// Percentage property
    #[zbus(property)]
    fn percentage(&self) -> Result<f64>;

    /// BatteryLevel property
    #[zbus(property)]
    fn battery_level(&self) -> Result<u32>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> Result<u32>;

    /// Type property
    #[zbus(property)]
    fn type_(&self) -> Result<u32>;
}
