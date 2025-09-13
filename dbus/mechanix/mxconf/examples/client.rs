use anyhow::Result;
use mxconf_dbus::get_setting;

/// Make sure you have these in your Cargo.toml:
/// zbus = "3"
/// tokio = { version = "1", features = ["full"] }
/// tracing = "0.1"

#[tokio::main]
async fn main() -> Result<()> {
    // Example: query a config key from the D-Bus service.
    let key = "org.mechanix.keyboard.general.layout";

    // Call your get_setting function.
    match get_setting(key).await {
        Ok(map) => {
            println!("Received values for key '{}':", key);
            for (k, v) in map {
                println!("  {} = {}", k, v);
            }
        }
        Err(e) => {
            eprintln!("Error getting setting: {:?}", e);
        }
    }

    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
