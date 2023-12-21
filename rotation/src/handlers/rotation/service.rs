use std::path;

use anyhow::Result;
use command::execute_command;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{
    handlers::wlroots::handler::{RotationDirection, WlrootsHandlerMessage},
    settings::{update_settings, RotationSettings},
};

pub struct DispatchRotationParams {
    pub wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>,
}

pub fn change_rotation(
    mode: &str,
    persist: bool,
    mut rotation_settings: &mut RotationSettings,
) -> Result<bool> {
    info!("change mode: {} persist: {}", mode, persist);

    let rotation_enabled_op = match mode {
        "enable" => Some(true),
        "disable" => Some(false),
        _ => None,
    };

    match rotation_enabled_op {
        Some(rotation_enabled) => {
            rotation_settings.rotation.enabled = rotation_enabled;
            if persist {
                match update_settings(rotation_settings.clone()) {
                    Ok(v) => {
                        info!("settings updated {}", v);
                    }
                    Err(e) => {
                        error!("failed to update settings {}", e)
                    }
                };
            }
        }
        None => (),
    }

    Ok(true)
}

pub fn set_default_rotation(mut settings: &mut RotationSettings) -> Result<bool> {
    let mode = match settings.rotation.enabled {
        true => "enable",
        false => "disable",
    };

    let _ = change_rotation(mode, false, settings);

    Ok(true)
}

pub async fn dispatch_rotation_status(
    dispatch_rotation_params: DispatchRotationParams,
) -> Result<bool> {
    //get accelerometer readings here decide rotation and send event

    let (x, y, z) = (10, 10, 10);

    let _ = dispatch_rotation_params
        .wlroots_sender_tx
        .send(WlrootsHandlerMessage::Rotate(RotationDirection::Right))
        .await;

    Ok(true)
}
