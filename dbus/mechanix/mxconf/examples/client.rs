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
    let key = "org.mechanix.keyboard.general.layout";
    let schema = "org.mechanix.keyboard";
    // Call your get_setting function.
    match watch_setting(schema, None).await {
        Ok(mut stream) => {
            println!("Watching for changes to {}...", key);
            tokio::spawn(async move {
                while let Some(signal) = stream.next().await {
                    if let Ok((_schema, signal_key, value)) = signal.body::<(String, String, String)>() {
                        println!("Received change signal for key: {}", signal_key);
                    } else {
                        println!("Failed to parse signal body for key: {:?}", key);
                    }
                }
            });
        }
        Err(e) => println!("Failed to watch for changes to {}: {}", key, e),
    }


    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
