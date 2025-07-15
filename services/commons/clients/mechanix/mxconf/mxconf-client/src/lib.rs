use anyhow::Result;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
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

/// Get a setting from the server and display it in a table
pub async fn get_setting_table(key: &str) -> Result<(), anyhow::Error> {
    debug!("Calling get_setting_table for key: {}", key);
    // Get the setting value
    let values = get_setting(key).await?;
    // Create a table for displaying the setting
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Key", "Value"]);

    // Add each setting as a row in the table
    for (k, v) in &values {
        table.add_row(vec![k, v]);
    }

    // Print the table
    println!("{table}");

    Ok(())
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

/// Set a setting on the server and display the result in a table
pub async fn set_setting_table(key: &str, value: &str) -> Result<(), anyhow::Error> {
    info!("Calling set_setting_table for key: {}", key);
    // Set the setting and get the result
    let result = set_setting(key, value).await?;
    info!("Set value for key {}: {} result: {}", key, value, result);

    // Create a table for displaying the result
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Key", "Value", "Result"]);

    // Add the setting as a row in the table
    table.add_row(vec![key, value, &result]);

    // Print the table
    println!("{table}");

    Ok(())
}

/// Watch for changes to a setting
pub async fn watch_setting(key: &str) -> Result<(), anyhow::Error> {
    info!("Connecting to D-Bus session for watch_setting");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Watching for changes to key: {}", key);
    // Only listen to signals where key matches the provided key
    let mut stream = proxy
        .receive_signal_with_args("SchemaKeyChanged", &[(0, key)])
        .await?;

    // Create a table for the initial message
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Status"]);
    table.add_row(vec![format!("Watching for changes to key: {}", key)]);
    println!("{table}");

    // Process signals as they come in
    while let Some(signal) = stream.next().await {
        if let Ok((signal_key, )) = signal.body::<(String,)>() {
            info!("Received change signal for key: {}", signal_key);
            // Get the new value
            let value = get_setting(&signal_key).await?;

            // Create a table for the change notification
            let mut change_table = Table::new();
            change_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Key", "New Value"]);
            for (k, v) in &value {
                change_table.add_row(vec![k, v]);
            }
            println!("{change_table}");
        } else {
            error!("Failed to parse signal body for key: {}", key);

            // Create a table for the error message
            let mut error_table = Table::new();
            error_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Error"]);
            error_table.add_row(vec!["Failed to parse signal body."]);
            println!("{error_table}");
        }
    }

    Ok(())
}

/// List all available schemas
pub async fn list_schemas() -> Result<(), anyhow::Error> {
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

    // Create a table for displaying schemas
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Available Schemas"]);

    // Add each schema as a row in the table
    for schema in schemas {
        table.add_row(vec![schema]);
    }

    // Print the table
    println!("{table}");

    Ok(())
}

pub async fn list_keys(schema: &str) -> Result<(), anyhow::Error> {
    info!("Connecting to D-Bus session for list_keys");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Listing keys for schema: {}", schema);
    // Get the list of keys
    let keys = proxy.list_keys(schema).await?;
    debug!("Received keys for schema {}: {:?}", schema, keys);

    // Create a table for displaying keys
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![format!("Keys in Schema: {}", schema)]);

    // Add each key as a row in the table
    for key in keys {
        table.add_row(vec![key]);
    }

    // Print the table
    println!("{table}");

    Ok(())
}
pub async fn describe_key(schema: &str, key: &str) -> Result<(), anyhow::Error> {
    info!("Connecting to D-Bus session for describe_key");
    let connection = Connection::session().await?;

    // Create a proxy for the ConfigServer interface
    let proxy = ConfigServerProxy::new(&connection).await?;

    info!("Describing key: {} in schema: {}", key, schema);
    // Get the key description
    let description = proxy.describe_key(schema, key).await?;
    debug!("Received description for key {} in schema {}: {}", key, schema, description);

    // Create a table for displaying the key description
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Schema", "Key", "Description"]);

    // Add the description as a row in the table
    table.add_row(vec![schema, key, &description]);

    // Print the table
    println!("{table}");

    Ok(())
}
