use std::{ collections::HashSet, fs };
use anyhow::Result;
use toml;
// use extensions::device::Device;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Extension {
    unique_id: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExtensionConfig {
    extensions: Vec<Extension>,
}
#[derive(Debug, Deserialize)]
pub struct Device {
    unique_id: String,
}
fn main() -> Result<()> {
    // Choose one loader depending on TOML shape

    // let current_id = "xyz789";
    let device = Device { unique_id: "xyz789".to_string() };
    dbg!(is_extension(device));
    Ok(())
}

fn is_extension(device: Device) -> bool {
    let id = device.unique_id;
    match fs::read_to_string("./config.toml") {
        Ok(raw) => {
            match toml::from_str::<ExtensionConfig>(&raw) {
                Ok(cfg) => {
                    let ids: Vec<String> = cfg.extensions
                        .into_iter()
                        .map(|ext| ext.unique_id)
                        .collect();
                    ids.contains(&id)
                }
                Err(e) => {
                    println!("Error parsing TOML: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            false
        }
    }
}
