use anyhow::Result;
use log::{debug, error, info};
use zbus::export::ordered_stream::OrderedStreamExt;

pub struct DisplayInterface {}
impl DisplayInterface {
    pub fn new() -> DisplayInterface {
        Self {}
    }

    /// Watches for changes to the brightness setting via D-Bus.
    ///
    /// Listens for brightness change signals from the `org.mechanix.settings` schema,
    /// parses the new value, and applies it using `mechanix_system_dbus::set_brightness`.
    /// Logs changes and errors encountered during the process.
    pub async fn watch_brightness(&self) -> Result<()> {
        info!("Watching brightness");
        let schema = "org.mechanix.settings";
        let key = "brightness.value";
        match mxconf_dbus::watch_setting(schema, Some(key.to_string())).await {
            Ok(mut stream) => {
                // Process signals as they come in
                while let Some(signal) = stream.next().await {
                    if let Ok((_schema, signal_key, value)) =
                        signal.body::<(String, String, String)>()
                    {
                        debug!("Received change signal for key: {}", signal_key);
                        let brightness: u8 = match value.parse() {
                            Ok(v) => v,
                            Err(e) => {
                                error!("Failed to parse brightness value: {}", e);
                                continue;
                            }
                        };
                        match system_dbus::set_brightness(brightness).await {
                            Ok(()) => {
                                info!("Brightness set to: {}", brightness);
                            }
                            Err(e) => {
                                error!("Failed to set brightness: {}", e);
                            }
                        }
                    } else {
                        error!("Failed to parse signal body for key: {:?}", schema);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error getting stream for brightness: {:?}", e);
            }
        }
        Ok(())
    }
}
