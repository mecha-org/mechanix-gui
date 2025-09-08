// pub mod extension;
pub mod types;
use zvariant::{ Type };
use serde::{ Serialize, Deserialize };
use types::{ DeviceType, ConnectionType };
use evdevil::Evdev;
use evdev::EventSummary::Key;
use std::path::PathBuf;
use std::{ collections::HashSet, fs };

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Device {
    pub path: PathBuf,
    pub device_type: DeviceType,
    pub connection_type: ConnectionType,
    pub connection_key: String,
    pub name: String,
    pub vendor_id: u16,
    pub unique_id: String,
}

impl Device {
    pub fn new(evdev: Evdev) -> Self {
        let path = evdev.path().to_path_buf();
        let (device_type,connection_key) = DeviceType::get_device_type(&evdev);
        let connection_type = ConnectionType::get_connection_type(&evdev);
        let name = evdev
            .name()
            .ok()
            .unwrap_or_else(|| "Unknown".to_string());
        let vendor_id = evdev
            .input_id()
            .ok()
            .map(|id| id.vendor())
            .unwrap_or(0);
        let unique_id = evdev
            .unique_id()
            .ok()
            .flatten()
            .unwrap_or_else(|| "Unknown".to_string());

        Self {
            path,
            device_type,
            connection_type,
            connection_key,
            name,
            vendor_id,
            unique_id,
        }
    }

    pub fn evdev(&self) -> Option<Evdev> {
        let evdev = match Evdev::open(&self.path) {
            Ok(evdev) => { Some(evdev) }
            Err(_) => { None }
        };
        evdev
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }

    pub fn connection_type(&self) -> &ConnectionType {
        &self.connection_type
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn vendor_id(&self) -> &u16 {
        &self.vendor_id
    }

    pub fn unique_id(&self) -> &String {
        &self.unique_id
    }

    pub fn connection_key(&self) -> &String {
        &self.connection_key
    }

    pub fn is_extension(&self) -> (bool, &String) {
        (*self.device_type() == DeviceType::Extension, &self.connection_key)
    }
}
