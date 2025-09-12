use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use anyhow::{bail, Context};
use log::{error, info, warn};
use zbus::{fdo::Error as ZbusError, dbus_interface};


/// ConfigServerInterface struct for D-Bus interface.
///
/// This struct implements the D-Bus interface for the configuration server.
/// It provides methods for listing schemas, listing keys, describing keys,
/// getting settings, and setting settings. It also emits signals when
/// settings are changed.
///
/// The interface is served at the path defined by the SERVED_AT constant.
#[derive()]
pub struct DisplayInterface {
    pub path: String,
}

#[dbus_interface(name = "org.mechanix.services.Display")]
impl DisplayInterface {
    pub fn get_brightness(&self) -> Result<u8, ZbusError> {
        info!("init");
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open display brightness file: {}", e);
                return Err(ZbusError::Failed("display brightness file not found".to_string()));
            }
        };

        let buffer = BufReader::new(file);
        let buffer_value = match buffer.lines().next() {
            None => {
                error!("There is no content in the file");
                return Err(ZbusError::Failed("Failed to read display brightness file".to_string()));
            }
            Some(res) => {
                match res {
                    Ok(value) => value,
                    Err(e) => {
                        error!("Failed to read display brightness file: {}", e);
                        return Err(ZbusError::Failed("Failed to read display brightness file".to_string()));
                    }
                }
            }
        };

        let value = match buffer_value
            .trim() // Use the ? operator to extract the String and propagate errors if any.
            .parse::<u8>() {
            Ok(value) => value,
            Err(e) => {
                error!("Failed to parse display brightness value: {}", e);
                return Err(ZbusError::Failed("Failed to parse display brightness value".to_string()));
            }
        };
        info!("brightness: {}", value);
        Ok(value)
    }
    pub fn set_brightness(&self, brightness: u8) -> Result<(), ZbusError> {
        // Check if the brightness value is valid
        if brightness > 254 {
            warn!("invalid brightness value: {brightness}");
            return Err(ZbusError::Failed("invalid brightness value, value must be between 0 and 254".to_string()));
        }

        let mut file = match File::create(&self.path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open display brightness file: {}", e);
                return Err(ZbusError::Failed("display brightness file not found".to_string()));
            }
        };

        match file.write_all(brightness.to_string().as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to write display brightness file: {}", err);
                return Err(ZbusError::Failed("Failed to write display brightness file".to_string()));
            }
        }
        info!("brightness set to: {}", brightness);
        Ok(())
    }
}
