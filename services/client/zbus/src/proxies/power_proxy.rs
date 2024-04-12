use tracing::info;
use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Power",
    default_service = "org.mechanix.services.Power",
    default_path = "/org/mechanix/services/Power"
)]
trait PowerBusInterface {
    async fn get_battery_status(&self) -> Result<String>;
    async fn get_battery_percentage(&self) -> Result<f32>;
    async fn get_screen_timeout(&self) -> Result<u32>;
    async fn get_cpu_governor(&self) -> Result<String>;
    async fn set_screen_timeout(&self, value: u32) -> Result<u32>;
    async fn set_cpu_governor(&self, value: &str) -> Result<String>;
}

pub struct Power;

impl Power {
    pub async fn get_battery_status() -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_battery_status().await?;
        Ok(reply)
    }

    pub async fn get_screen_timeout() -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_screen_timeout().await?;
        let result = format!("{}s", reply);
        Ok(result)
    }

    pub async fn get_performance_mode() -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_cpu_governor().await?;
        let result = match reply.as_str() {
            "performance" => "High",
            "powersave" => "Low",
            "ondemand" => "Balanced",
            _ => "",
        };
        Ok(result.to_string())
    }

    pub async fn set_screen_timeout(value: u32) -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = match proxy.set_screen_timeout(value).await {
            Ok(value) => value,
            Err(e) => {
                print!("error {:?}", e);
                0
            }
        };
        let result = format!("{}s", reply);
        Ok(result)
    }

    pub async fn set_cpu_governor(value: &str) -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;

        let value_map = match value {
            "High" => "performance",
            "Low" => "powersave",
            "Balanced" => "ondemand",
            _ => "",
        };

        let reply = proxy.set_cpu_governor(value_map).await?;
        Ok(reply)
    }

    pub async fn get_battery_percentage() -> Result<u8> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_battery_percentage().await?;
        Ok(reply.round() as u8)
    }
}
