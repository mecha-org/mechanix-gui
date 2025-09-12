use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    // Example: query a config key from the D-Bus service.
    // Call your get_setting function.
    match system_dbus::get_brightness().await {
        Ok(brightness) => {
            println!("Received brightness '{}':", brightness);
        }
        Err(e) => {
            eprintln!("Error getting setting: {:?}", e);
        }
    }

    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
