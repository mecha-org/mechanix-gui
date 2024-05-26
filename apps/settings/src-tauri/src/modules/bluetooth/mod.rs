pub mod client;
pub mod listener;
use crate::error::Error;
use serde::Serialize;

use self::client::BluetoothScanResponse;

#[derive(Debug, Serialize)]
pub struct BluetoothData {
    pub status: i8, 
    pub available_devices: Vec<BluetoothScanResponse>
}

#[tauri::command]
pub async fn get_bluetooth_status() -> Result<BluetoothData, Error> {
    println!("get_bluetooth_status....");
    let bluetooth_on = match client::BluetoothService::status().await {
        Ok(v) => v,
        Err(e) => {
            println!("bluetooth bluetooth_on error {:?} ", e);
            return Err(Error::Other(e.to_string()))
        },
    };

    if bluetooth_on == 0 {
        return Ok(BluetoothData{
            status: 0,
            available_devices: vec![]
        });
    };

    let scan_response = match client::BluetoothService::scan().await {
        Ok(v) => v,
        Err(e) => {
            println!("bluetooth scan_response error {:?} ", e);
            return Err(Error::Other(e.to_string()))
        },
    };

    println!("scan_response {:?}", scan_response);

    if scan_response.bluetooth_devices.len() > 0 {
        return Ok(BluetoothData{
            status: 1,
            available_devices: scan_response.bluetooth_devices
        });
    } else {
        return Ok(BluetoothData{
            status: 0,
            available_devices: vec![]
        });
    };
}


#[tauri::command]
pub async fn update_enable_bluetooth() -> Result<(), Error> {
    println!("update_enable_bluetooth called....");
    match client::BluetoothService::enable_bluetooth().await {
        Ok(result) => {
            println!("RESULT enable_bluetooth: {:?}", result);
            Ok(())
        },
        Err(e) => {
            println!("CHECK enable_bluetooth ERROR: {:?}", e);
            Err(Error::Other(e.to_string()))
        }
    }
}

#[tauri::command]
pub async fn update_disable_bluetooth() -> Result<(), Error> {
    println!("update_disable_bluetooth called....");
    match client::BluetoothService::disable_bluetooth().await {
        Ok(result) => {
            println!("RESULT disable_bluetooth: {:?}", result);
            Ok(())
        },
        Err(e) => {
            println!("CHECK disable_bluetooth ERROR: {:?}", e);
            Err(Error::Other(e.to_string()))
        }
    }
}