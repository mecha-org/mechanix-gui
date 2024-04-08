use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection,
};

use mechanix_power_ctl::Power;
use zbus_polkit::policykit1::{AuthorityProxy, CheckAuthorizationFlags, Subject};

pub struct PowerBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `BatteryInfoResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BatteryInfoResponse {
    pub vendor: String,
    pub model: String,
    pub serial_number: String,
    pub technology: String,
    pub state: String,
}

#[interface(name = "org.mechanix.services.Power")]
impl PowerBusInterface {
    pub async fn get_battery_status(&self) -> Result<String, ZbusError> {
        let power = Power::new();
        let status = power.get_battery_status();
        Ok(status)
    }

    pub async fn get_battery_info(&self) -> Result<BatteryInfoResponse, ZbusError> {
        let power = Power::new();
        let info = power.get_battery_info();
        Ok(BatteryInfoResponse {
            vendor: info.vendor.unwrap_or("").to_string(),
            model: info.model.unwrap_or("").to_string(),
            serial_number: info.serial_number.unwrap_or("").to_string(),
            technology: info.technology,
            state: info.state,
        })
    }

    //all battery info
    pub async fn info(&self) -> Result<String, ZbusError> {
        let power = Power::new();
        let info = power.info();
        Ok(info)
    }

    //get battery percentage
    pub async fn get_battery_percentage(&self) -> Result<f32, ZbusError> {
        let power = Power::new();
        let percentage = power.get_battery_percentage();
        Ok(percentage)
    }

    //set cpu governor
    pub async fn set_cpu_governor(&self, governor: &str) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.set_cpu_governor(governor);
        Ok(())
    }

    //get cpu governor
    pub async fn get_cpu_governor(&self) -> Result<Vec<String>, ZbusError> {
        let governor = Power::get_available_governors();
        Ok(governor.unwrap_or(vec![]))
    }

    //get current cpu governor
    pub async fn get_current_cpu_governor(&self) -> Result<String, ZbusError> {
        let governor = match Power::get_current_cpu_governor() {
            Ok(governor) => governor,
            Err(_) => "".to_string(),
        };

        Ok(governor)
    }

    //get cpu frequency
    pub async fn get_cpu_frequency(&self) -> Result<String, ZbusError> {
        let frequency = match Power::get_cpu_frequency() {
            Ok(frequency) => frequency,
            Err(_) => "".to_string(),
        };
        Ok(frequency)
    }

    #[cfg(feature = "auth")]
    pub async fn power_off(&self) -> Result<(), ZbusError> {
        let _ = match authorized().await.unwrap() {
            true => {
                let power = Power::new();
                let _ = power.power_off();
            }
            false => return Err(ZbusError::Failed("Not authorized".to_string())),
        };

        Ok(())
    }

    #[cfg(not(feature = "auth"))]
    pub async fn power_off(&self) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.power_off();
        Ok(())
    }

    #[cfg(feature = "auth")]
    pub async fn reboot(&self) -> Result<(), ZbusError> {
        let _ = match authorized().await.unwrap() {
            true => {
                let power = Power::new();
                let _ = power.reboot();
            }
            false => return Err(ZbusError::Failed("Not authorized".to_string())),
        };
        Ok(())
    }

    #[cfg(not(feature = "auth"))]
    pub async fn reboot(&self) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.reboot();
        Ok(())
    }

    #[cfg(feature = "auth")]
    pub async fn suspend(&self) -> Result<(), ZbusError> {
        let _ = match authorized().await.unwrap() {
            true => {
                let power = Power::new();
                let _ = power.suspend();
            }
            false => return Err(ZbusError::Failed("Not authorized".to_string())),
        };
        Ok(())
    }

    #[cfg(not(feature = "auth"))]
    pub async fn suspend(&self) -> Result<(), ZbusError> {
        let power = Power::new();
        let _ = power.suspend();
        Ok(())
    }
}

async fn authorized() -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;
    let proxy = AuthorityProxy::new(&connection).await?;
    let subject = Subject::new_for_owner(std::process::id(), None, None)?;
    let result = proxy
        .check_authorization(
            &subject,
            "org.mechanix.services.Power",
            &std::collections::HashMap::new(),
            CheckAuthorizationFlags::AllowUserInteraction.into(),
            "",
        )
        .await?;
    Ok(result.is_authorized)
}
