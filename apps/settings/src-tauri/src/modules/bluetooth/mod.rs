pub mod client;
pub mod listener;
use crate::error::Error;

use self::client::BluetoothScanListResponse;

#[tauri::command]
pub async fn get_bluetooth_status() -> Result<i8, Error> {
    println!("get_bluetooth_status....");
    match client::BluetoothService::status().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}


#[tauri::command]
pub async fn scan_bluetooth() -> Result<BluetoothScanListResponse, Error> {
    println!("scan_bluetooth....");
    match client::BluetoothService::scan().await {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("bluetooth::scan_response error {:?} ", e);
            return Err(Error::Other(e.to_string()))
        },
    };
}


#[tauri::command]
pub async fn enable_bluetooth() -> Result<(), Error> {
    println!("enable_bluetooth called....");
    match client::BluetoothService::enable_bluetooth().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    }
}

// #[tauri::command]
// pub async fn enable_bluetooth() -> Result<bool, Error> {
//     println!("enable_bluetooth called....");
//     match client::BluetoothService::enable_bluetooth().await {
//         Ok(v) => {
//             // return Ok(v)
//             println!("enable_bluetooth result: {:?}", v);
//             return Ok(true);
//         },
//         Err(e) => {
//             println!("enable_bluetooth error: {:?}", e.to_string());
//             return Err(Error::Other(e.to_string()))
//         }
//     }
// }

#[tauri::command]
pub async fn disable_bluetooth() -> Result<(), Error> {
    println!("disable_bluetooth called....");
    match client::BluetoothService::disable_bluetooth().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    }
}

#[tauri::command]
pub async fn connect_bluetooth_device(address: &str) -> Result<(), Error> {
    println!("connect_bluetooth_device called.... {:?}", address.to_owned());
    match client::BluetoothService::connect(address).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    }
}

#[tauri::command]
pub async fn disconnect_bluetooth_device(address: &str) -> Result<(), Error> {
    println!("disconnect_bluetooth_device called.... {:?}", address.to_owned());
    match client::BluetoothService::disconnect(&address).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    }
}