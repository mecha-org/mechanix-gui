use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub network: Network,
    pub display: Display,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Network {
    pub device: String,
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_configs_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-c" || args[1] == "--configs") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

/// # Reads Settings YML
///
/// Reads the `services-config` and parsers to BaseConfig
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_configs_yml() -> Result<BaseConfig> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_SERVICES_CONFIG_PATH").unwrap_or(String::from("services-config.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_configs_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!(
        task = "read_configs_yml",
        "configs file location - {:?}", file_path
    );

    //open file
    let config_file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(anyhow!("Error opening file: {:?}", e));
        }
    };

    // Parse the file
    let config: BaseConfig = match serde_yaml::from_reader(config_file) {
        Ok(config) => config,
        Err(e) => {
            return Err(anyhow!("Error parsing file: {:?}", e));
        }
    };

    Ok(config)
}
