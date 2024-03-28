
use tracing::info;
use zbus::{Connection, proxy , Result,};


#[proxy(
    interface = "Mechanix.Services.Power",
    default_service = "mechanix.services.power",
    default_path = "/org/mechanix/services/power"
)]
trait PowerBusInterface {
    async fn get_battery_status(&self) -> Result<u8>;
    async fn get_screen_timeout(&self) -> Result<u32>;
    async fn get_cpu_governor(&self) -> Result<String>;
    async fn set_screen_timeout(&self, value: u32) ->Result<u32>;
    async fn set_cpu_governor(&self, value: &str) ->Result<String>;

    


}

pub struct Power; 

impl Power {
    pub async fn get_battery_status() -> Result<u8> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_battery_status().await?;
        info!("get_battery_status reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn get_screen_timeout() -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_screen_timeout().await?;
        info!("get_screen_timeout reply: {:?}", reply);
        let result = format!("{}s", reply);
        Ok(result)
    }

    pub async fn get_performance_mode() -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_cpu_governor().await?;
        let result  = match reply.as_str() {
            "performance" => "High",
            "powersave" => "Low",
            "ondemand" => "Balanced",
            _=> ""
        };
        info!("get performance reply: {:?}", reply);
        Ok(result.to_string())
    }

    pub async fn set_screen_timeout(value: u32) -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = match proxy.set_screen_timeout(value).await {
            Ok(value)=> value,
            Err(e)=>{
                print!("error {:?}",e);
                0
            }
        };
        let result = format!("{}s", reply);
        Ok(result)
    }

    pub async fn set_cpu_governor(value: &str) -> Result<String> {

        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;

        let value_map  = match value {
            "High" => "performance",
            "Low" => "powersave",
            "Balanced" => "ondemand",
            _=> ""
        };

        let reply =  proxy.set_cpu_governor(value_map).await?;
        info!("get performance reply: {:?}", reply);
        Ok(reply)
    }
}
