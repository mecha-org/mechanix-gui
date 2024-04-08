
use tracing::info;
use zbus::{Connection, proxy , Result,};


#[proxy(
    interface = "org.mechanix.services.Power",
    default_service = "org.mechanix.services.Power",
    default_path = "/org/mechanix/services/Power"
)]
trait PowerBusInterface {
    async fn get_battery_percentage(&self) -> Result<f32>;
    // async fn get_screen_timeout(&self) -> Result<u32>;
    async fn get_current_cpu_governor(&self) -> Result<String>;
    // async fn set_screen_timeout(&self, value: u32) ->Result<u32>;
    async fn set_cpu_governor(&self, value: &str) ->Result<String>;

    


}

pub struct Power; 

impl Power {
    pub async fn get_battery_percentage() -> Result<f32> {
        println!(":::::get_battery_percentage reply: ");
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_battery_percentage().await?;
        println!(":::::get_battery_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn get_performance_mode() -> Result<String> {
        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_current_cpu_governor().await?;
        let result  = match reply.as_str() {
            "performance\n" => "High",
            "powersave\n" => "Low",
            "conservative\n" => "Balanced",
            _=> ""
        };
        println!("get performance reply: {:?}", reply);
        Ok(result.to_string())
    }


    pub async fn set_cpu_governor(value: &str) -> Result<String> {

        let connection = Connection::system().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;

        let value_map  = match value {
            "High" => "performance",
            "Low" => "powersave",
            "Balanced" => "conservative",
            _=> ""
        };

        let reply =  proxy.set_cpu_governor(value_map).await?;
        println!("get performance reply: {:?}", reply);
        Ok(reply)
    }
}
