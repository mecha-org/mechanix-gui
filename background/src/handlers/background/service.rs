use std::path;

use anyhow::Result;
use command::execute_command;
use tracing::{error, info};

use crate::settings::{update_settings, BackgroundSettings};

pub fn change_background(
    path: &str,
    mode: &str,
    persist: bool,
    mut background_settings: BackgroundSettings,
) -> Result<bool> {
    info!(
        "change bg path: {} mode: {} persist: {}",
        path, mode, persist
    );

    let command = "swww";
    let mut args = vec!["img", path];

    if mode.len() > 0 {
        args = [args, ["--transition-type", mode].to_vec()].concat();
    }

    match execute_command(command, &args) {
        Ok(v) => {
            info!("res from  execute_command {}", v);
            if persist {
                background_settings.background.path = Some(String::from(path));
                match update_settings(background_settings) {
                    Ok(v) => {
                        info!("settings updated {}", v);
                    }
                    Err(e) => {
                        error!("failed to update settings {}", e)
                    }
                };
            }
        }
        Err(e) => {
            error!("error while executing command {}", e);
        }
    };

    Ok(true)
}

pub fn set_default_background(settings: BackgroundSettings) -> Result<bool> {
    match settings.background.path.clone() {
        Some(path) => {
            let _ = change_background(
                &path,
                &settings.background.mode.clone().unwrap_or(String::from("")),
                false,
                settings.clone(),
            );
        }
        None => {
            info!("background path not found in setting")
        }
    }

    Ok(true)
}
