
use tracing::info;
use zbus::{Connection, proxy , Result, zvariant::{ DeserializeDict,Type}};
use serde::Serialize;

#[derive(DeserializeDict, Serialize, Debug, Type, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct WirelessInfoResponse {
    pub mac: String,
    pub frequency: String,
    pub signal: String,
    pub flags: String,
    pub name: String,
}



#[derive(DeserializeDict, Serialize, Type, Debug, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct WirelessScanListResponse {
    pub wireless_network: Vec<WirelessInfoResponse>,
}

#[derive(DeserializeDict, Serialize, Type, Debug, Clone, Default)]
/// A known WiFi network.
#[zvariant(signature = "a{sv}")]
pub struct KnownNetworkResponse {
    pub network_id: String,
    pub ssid: String,
    pub flags: String,
}

#[derive(DeserializeDict, Serialize, Type, Debug, Clone, Default)]
/// A known WiFi networkList
#[zvariant(signature = "a{sv}")]
pub struct KnownNetworkListResponse {
    pub known_network: Vec<KnownNetworkResponse>,
}


#[proxy(
    interface = "org.mechanix.services.Wireless",
    default_service = "org.mechanix.services.Wireless",
    default_path = "/org/mechanix/services/Wireless"
)]
trait Wireless {
    async fn scan(&self) -> Result<WirelessScanListResponse>;
    async fn known_networks(&self) -> Result<KnownNetworkListResponse>;
    async fn select_network(&self, network_id: &str) ->Result<()>;

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

    pub async fn known_networks() -> Result<KnownNetworkListResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.known_networks().await?;
        println!("known_networks  reply: {:?}", reply);
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
        println!("wifi_status reply: {:?}", reply);
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

    pub async fn connect_to_known_network(network_id: &str) -> Result<()> {

        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.select_network(network_id).await?;
        println!("get performance reply: {:?}", reply);
        Ok(())
    }

    pub async fn disconnect(value: &str) -> Result<()> {

        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply =  proxy.disconnect(value).await?;
        println!("get disconnect reply: {:?}", reply);
        Ok(reply)
    }
}
