use anyhow::Result;
use futures_util::stream::StreamExt;
use log::{debug, error, info};
use serde_json;
use std::collections::HashMap;
use zbus::{dbus_proxy, Connection};

#[dbus_proxy(
    interface = "org.mechanix.MxConf",
    default_service = "org.mechanix.MxConf",
    default_path = "/org/mechanix/MxConf"
)]
pub trait ConfigServer {
    /// Get a setting from the database
    async fn get_setting(&self, key: &str) -> zbus::Result<HashMap<String, String>>;

    /// Set a setting in the database
    async fn set_setting(&self, key: &str, value: &str) -> zbus::Result<String>;

    /// List all available schemas
    async fn list_schemas(&self) -> zbus::Result<String>;
    async fn list_keys(&self, schema: &str) -> zbus::Result<Vec<String>>;
    async fn describe_key(&self, schema: &str, key: &str) -> zbus::Result<String>;

    #[dbus_proxy(signal)]
    fn schema_key_changed(&self) -> zbus::Result<zbus::SignalStream>;
}

/// Get a setting from the server
pub async fn get_setting(key: &str) -> Result<HashMap<String, String>, anyhow::Error> {
    debug!("Connecting to D-Bus session for get_setting");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Getting setting for key: {}", key);
    // Get the setting
    let value = proxy.get_setting(key).await?;
    debug!("Received key and value respectively: {:?}", value);

    Ok(value)
}

/// Set a setting on the server
pub async fn set_setting(key: &str, value: &str) -> Result<String, anyhow::Error> {
    info!("Connecting to D-Bus session for set_setting");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Setting value for key: {} to {}", key, value);
    // Set the setting
    let result = proxy.set_setting(key, value).await?;
    debug!("Set result for key {}: {}", key, result);

    Ok(result)
}

/// Watch for changes to a setting
pub async fn watch_setting(key: &str) -> Result<zbus::SignalStream, anyhow::Error> {
    info!("Connecting to D-Bus session for watch_setting");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Watching for changes to key: {}", key);
    // Only listen to signals where key matches the provided key
    let stream = proxy
        .receive_signal_with_args("SchemaKeyChanged", &[(0, key)])
        .await?;

    Ok(stream)
}

/// List all available schemas
pub async fn list_schemas() -> Result<Vec<String>, anyhow::Error> {
    info!("Connecting to D-Bus session for list_schemas");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Listing all schemas");
    // Get the list of schemas
    let schemas_json = proxy.list_schemas().await?;
    debug!("Received schemas JSON: {}", schemas_json);

    // Parse the JSON string
    let schemas: Vec<String> = serde_json::from_str(&schemas_json)?;

    Ok(schemas)
}

pub async fn list_keys(schema: &str) -> Result<Vec<String>, anyhow::Error> {
    info!("Connecting to D-Bus session for list_keys");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Listing keys for schema: {}", schema);
    // Get the list of keys
    let keys = proxy.list_keys(schema).await?;
    debug!("Received keys for schema {}: {:?}", schema, keys);

    Ok(keys)
}
pub async fn describe_key(schema: &str, key: &str) -> Result<String, anyhow::Error> {
    info!("Connecting to D-Bus session for describe_key");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Describing key: {} in schema: {}", key, schema);
    // Get the key description
    let description = proxy.describe_key(schema, key).await?;
    debug!("Received description for key {} in schema {}: {}", key, schema, description);

    Ok(description)
}
