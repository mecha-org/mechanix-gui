pub mod client;

use crate::error::Error;
use serde::Serialize;

use self::client::{KnownNetworkListResponse, WirelessInfoResponse, WirelessScanListResponse, WirelessService};

#[derive(Debug, Serialize)]
pub struct WirelessData {
    pub status: bool, 
    pub connected_network: WirelessInfoResponse
}


#[tauri::command]
pub async fn get_wireless_status() -> Result<WirelessData, Error> {
    println!("get_wireless_status....");
    let wifi_on = match WirelessService::wifi_status().await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string())),
    };

    if wifi_on == false {
        return Ok(WirelessData{
            status: false,
            connected_network:  WirelessInfoResponse::default()
        });
    };

    let connected_network = match WirelessService::info().await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string())),
    };

    println!("scan_response {:?}", connected_network);
     return Ok(WirelessData{
            status: true,
            connected_network: connected_network
        });
    
}



#[tauri::command]
pub async fn enable_wifi() -> Result<bool, Error> {
    println!("Calling::wireless::enable_wifi()");
    match WirelessService::enable_wifi().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}

#[tauri::command]
pub async fn disable_wifi() -> Result<bool, Error> {
    println!("Calling::wireless::disable_wifi()");
    match WirelessService::disable_wifi().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}



#[tauri::command]
pub async fn get_connected_wifi_info() -> Result<WirelessInfoResponse, Error> {
    println!("Calling::wireless::get_connected_wifi_info()");
    match WirelessService::info().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}


#[tauri::command]
pub async fn get_wifi_status() -> Result<bool, Error> {
    println!("Calling::wireless::get_wifi_status()");
    match WirelessService::wifi_status().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}

#[tauri::command]
pub async fn wifi_scanning() -> Result<WirelessScanListResponse, Error> {
    println!("Calling::wireless::wifi_scanning()");
    match WirelessService::scan().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}

#[tauri::command]
pub async fn get_known_networks() -> Result<KnownNetworkListResponse, Error> {
    println!("Calling::wireless::get_known_networks()");
    match WirelessService::known_networks().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}

#[tauri::command]
pub async fn connect_to_network(ssid: &str, password: &str) -> Result<(), Error> {
    println!("Calling::wireless::connect_to_network()");
    match WirelessService::connect_to_network(ssid, password).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}

#[tauri::command]
pub async fn connect_to_known_network(network_id: &str) -> Result<(), Error> {
    println!("Calling::wireless::connect_to_known_network()");
    match WirelessService::connect_to_known_network(network_id).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
}