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

    pub async fn get_cpu_governor(&self) -> Result<String, ZbusError> {
        let power = Power::new();
        let governor = power.get_cpu_governor();
        Ok(governor)
    }

    pub async fn set_cpu_governor(&self, governor: String) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.set_cpu_governor(&governor);
        Ok(())
    }
}
