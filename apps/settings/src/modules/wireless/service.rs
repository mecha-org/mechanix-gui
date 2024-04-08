
use tracing::info;
use zbus::{Connection, proxy , Result, zvariant::{ DeserializeDict, SerializeDict,Type}};


#[derive(DeserializeDict, SerializeDict, Debug, Type, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct WirelessInfoResponse {
    pub mac: String,
    pub frequency: String,
    pub signal: String,
    pub flags: String,
    pub name: String,
}



#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct WirelessScanListResponse {
    pub wireless_network: Vec<WirelessInfoResponse>,
}



#[proxy(
    interface = "org.mechanix.services.Wireless",
    default_service = "org.mechanix.services.Wireless",
    default_path = "/org/mechanix/services/Wireless"
)]
trait Wireless {
    async fn scan(&self) -> Result<WirelessScanListResponse>;
    async fn info(&self) -> Result<WirelessInfoResponse>;
    async fn status(&self) -> Result<bool>;
    async fn enable(&self) -> Result<bool>;
    async fn disable(&self) -> Result<bool>;
    async fn connect(&self, ssid: &str, password: &str) ->Result<()>;
    async fn disconnect(&self, ssid: &str) ->Result<()>;

    


}

pub struct WirelessService; 

impl WirelessService {
    pub async fn scan() -> Result<WirelessScanListResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.scan().await?;
        println!("scan network reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn info() -> Result<WirelessInfoResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.info().await?;
        println!("current connected wifi : {:?}", reply);
        Ok(reply)
    }

    pub async fn wifi_status() -> Result<bool> {
        println!("In wireless status call:: ");
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply =  proxy.status().await?;
        println!("status reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn enable_wifi() -> Result<bool> {
        println!("In wireless status call:: ");
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply =  proxy.enable().await?;
        println!("enable_wifi reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn disable_wifi() -> Result<bool> {
        println!("In wireless status call:: ");
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply =  proxy.disable().await?;
        println!("disable_wifi reply: {:?}", reply);
        Ok(reply)
    }


    pub async fn connect_to_network(ssid: &str, password: &str) -> Result<()> {

        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.connect(ssid, password).await?;
        println!("get performance reply: {:?}", reply);
        Ok(())
    }


    pub async fn disconnect(value: &str) -> Result<()> {

        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.disconnect(value).await?;
        println!("get performance reply: {:?}", reply);
        Ok(reply)
    }
}
