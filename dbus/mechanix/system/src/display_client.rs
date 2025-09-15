use anyhow::Result;
use log::{debug, info};
use zbus::{dbus_proxy, Connection};

#[dbus_proxy(
    interface = "org.mechanix.services.Display",
    default_service = "org.mechanix.services.Display",
    default_path = "/org/mechanix/services/Display"
)]
pub trait DisplayServer {
    async fn set_brightness(&self, brightness: u8) -> Result<()>;
    async fn get_brightness(&self) -> Result<u8>;
}

/// Set the handlers brightness to the given value.
///
/// This function connects to the system bus and sends a D-Bus method call to the
/// `org.mechanix.services.Display` service, which sets the handlers brightness to
/// the given value. The method returns a `Result` that is `Ok` if the method call
/// was successful, or `Err` if the method call failed.
///
/// # Errors
///
/// If the method call fails, this function will return an `Err` containing an
/// `anyhow::Error` that describes the error.
pub async fn set_brightness(brightness: u8) -> Result<(), anyhow::Error> {
    info!("Connecting to system D-Bus for set brightness");
    let connection = Connection::system().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = DisplayServerProxy::new(&connection).await?;
    // Set the setting
    let _result = proxy.set_brightness(brightness).await?;
    Ok(())
}

/// Get the current handlers brightness.
///
/// This function connects to the system bus and sends a D-Bus method call to the
/// `org.mechanix.services.Display` service, which returns the current handlers
/// brightness as a `u8` value. The method returns a `Result` that is `Ok` if the
/// method call was successful, or `Err` if the method call failed.
///
/// # Errors
///
/// If the method call fails, this function will return an `Err` containing an
/// `anyhow::Error` that describes the error.
pub async fn get_brightness() -> Result<u8, anyhow::Error> {
    debug!("Connecting to system D-Bus for get brightness");
    let connection = Connection::system().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = DisplayServerProxy::new(&connection).await?;
    // Get the setting
    let value = proxy.get_brightness().await?;
    Ok(value)
}
