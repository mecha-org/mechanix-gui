use mechanix_power_ctl::Power;
use zbus::{fdo::Error as ZbusError, interface};

pub struct PowerBusInterface {}

#[interface(name = "org.mechanix.services.Power")]
impl PowerBusInterface {
    pub async fn session_logout(&self) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.session_logout();
        Ok(())
    }
}
