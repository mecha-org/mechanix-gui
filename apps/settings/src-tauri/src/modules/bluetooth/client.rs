
use serde::Serialize;
use tracing::info;
use zbus::{proxy, zvariant::{ DeserializeDict, SerializeDict, Type}, Connection, Result};

#[derive(DeserializeDict, Debug, Type, Clone, Default, Serialize)]
#[zvariant(signature = "a{sv}")]
pub struct BluetoothScanResponse {
    pub address: String,
    pub address_type: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub class: Option<u32>,
    pub rssi: Option<i16>,
    pub tx_power: Option<i16>,
    pub is_paired: bool,
    pub is_trusted: bool,
}


#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, Default)]
#[zvariant(signature = "a{sv}")]
pub struct BluetoothScanListResponse {
    pub bluetooth_devices: Vec<BluetoothScanResponse>,
}


#[proxy(
    interface = "org.mechanix.services.Bluetooth",
    default_service = "org.mechanix.services.Bluetooth",
    default_path = "/org/mechanix/services/Bluetooth"
)]
trait Bluetooth {
    async fn scan(&self) -> Result<BluetoothScanListResponse>;
    async fn enable(&self) -> Result<()>;
    async fn disable(&self) -> Result<()>;
    async fn status(&self) -> Result<i8>;
}

pub struct BluetoothService; 

impl BluetoothService {
    pub async fn scan() -> Result<BluetoothScanListResponse> {
        let connection = Connection::system().await?;
        let proxy: BluetoothProxy = BluetoothProxy::new(&connection).await?;
        let reply: BluetoothScanListResponse =  proxy.scan().await?;
        println!("scan bluetooth reply: {:?}", reply);
        Ok(reply)
    }
    
    pub async fn enable_bluetooth() -> Result<()> {
        info!("In bluetooth enable status call:: ");
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply =  proxy.enable().await?;
        println!("enable_bluetooth reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn disable_bluetooth() -> Result<()> {
        info!("In bluetooth disable status call:: ");
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply =  proxy.disable().await?;
        println!("disable_bluetooth reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn status() -> Result<i8> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply = proxy.status().await?;
        Ok(reply)
    }


}