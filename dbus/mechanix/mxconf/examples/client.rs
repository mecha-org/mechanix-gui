use anyhow::Result;
use futures_util::StreamExt;
use log::{error, info};
use mxconf_dbus::{get_setting, watch_setting};

/// Make sure you have these in your Cargo.toml:
/// zbus = "3"
/// tokio = { version = "1", features = ["full"] }
/// tracing = "0.1"

#[tokio::main]
async fn main() -> Result<()> {
    // Example: query a config key from the D-Bus service.
    // To register your schema, place a new TOML file in `/usr/share/mxconf/schemas`.
    // For more information, see the README.md file:
    // https://github.com/mecha-org/mechanix-gui/tree/pre-release/services/conf#-configuration-schema
    let registered_schema = "org.mechanix.launcher";
    let key_to_watch = "org.mechanix.launcher.theme.color";

    // Call your get_setting function.
    match watch_setting(registered_schema, Some(key_to_watch.to_string())).await {
        Ok(mut stream) => {
            println!("Watching for changes to {}...", key_to_watch);
            tokio::spawn(async move {
                while let Some(signal) = stream.next().await {
                    if let Ok((_schema, signal_key, value)) = signal.body::<(String, String, String)>() {
                        println!("Received change signal for key: {}", signal_key);
                    } else {
                        println!("Failed to parse signal body for key: {:?}", key_to_watch);
                    }
                }
            });
        }
        Err(e) => println!("Failed to watch for changes to {}: {}", key_to_watch, e),
    }


    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
