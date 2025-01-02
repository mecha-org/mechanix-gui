use anyhow::{bail, Result as AnyhowResult};
use tracing::info;
use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Power",
    default_service = "org.mechanix.services.Power",
    default_path = "/org/mechanix/services/Power"
)]
trait PowerBusInterface {
    async fn get_cpu_governor(&self) -> Result<Vec<String>>;
    async fn get_current_cpu_governor(&self) -> Result<String>;
    async fn set_cpu_governor(&self, value: &str) -> Result<()>;
}

pub struct Power;

impl Power {
    pub async fn get_all_performance_modes() -> Result<Vec<String>> {
        println!("battery::client::get_all_performance_modes()");
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let mut reply = proxy.get_cpu_governor().await?;
        println!("get_cpu_governor reply ====> {:?}", reply);
        for mode in reply.iter_mut() {
            if *mode == "performance" {
                *mode = "High".to_string();
            } else if *mode == "powersave" {
                *mode = "Low".to_string();
            } else if *mode == "conservative" {
                *mode = "Balanced".to_string();
            }
        }
        Ok(reply)
    }

    pub async fn get_battery_percentage() -> AnyhowResult<f64> {
        println!("battery::client::get_battery_percentage()");
        let battery = match upower::get_battery().await {
            Ok(battery) => battery,
            Err(e) => bail!(e.to_string()),
        };

        let percentage: f64 = match battery.percentage().await {
            Ok(p) => p,
            Err(e) => bail!(e.to_string()),
        };
        Ok(percentage)
    }

    pub async fn get_current_performance_mode() -> Result<String> {
        println!("battery::client::get_current_performance_mode()");
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_current_cpu_governor().await?;
        println!("get_current_performance_mode reply ====> {:?}", reply);
        let result = match reply.as_str() {
            "performance\n" => "High",
            "powersave\n" => "Low",
            "conservative\n" => "Balanced",
            _ => "",
        };
        Ok(result.to_string())
    }

    pub async fn set_cpu_governor(value: &str) -> Result<()> {
        println!("battery::client::set_cpu_governor()");
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let value_map = match value {
            "High" => "performance",
            "Low" => "powersave",
            "Balanced" => "conservative",
            _ => "",
        };
        println!("value_map : {:?}", value_map.to_string());
        let reply = proxy.set_cpu_governor(value_map).await?;
        Ok(())
    }
}
