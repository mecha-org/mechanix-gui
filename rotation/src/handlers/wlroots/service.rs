use std::process::Command;

use anyhow::{bail, Result};
use tokio::sync::mpsc;
use tracing::{error, info};
use wayland_client::protocol::wl_output::Transform;

use crate::{
    backends::{
        sway::SwayBackend, wlroots::WaylandBackend, xorg::XorgBackend, DisplayManager, Orientation,
    },
    handlers::rotation::errors::{RotationHandlerError, RotationHandlerErrorCodes},
    settings::RotationConfigs,
};

use super::handler::WlrootsHandlerMessage;

pub struct DispatchRotationParams {
    pub wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>,
}

pub fn rotate(orientation: Orientation, rotation_configs: RotationConfigs) -> Result<bool> {
    info!("rotate orientation:  {:?}", orientation);
    let mut backend: Box<dyn DisplayManager> = match WaylandBackend::new(&rotation_configs.display)
    {
        Ok(wayland_backend) => {
            if process_exists("sway") {
                Box::new(SwayBackend::new(
                    wayland_backend,
                    rotation_configs.disable_keyboard,
                ))
            } else {
                Box::new(wayland_backend)
            }
        }
        Err(e) => {
            if process_exists("Xorg") || process_exists("X") {
                Box::new(XorgBackend::new(
                    &rotation_configs.display,
                    rotation_configs.touchscreens.clone(),
                ))
            } else {
                bail!(RotationHandlerError::new(
                    RotationHandlerErrorCodes::UnsupportedCompositorError,
                    format!(
                        "Unable to find supported Xorg process or wayland compositor: {}",
                        e
                    ),
                    true
                ));
            }
        }
    };
    backend.change_rotation_state(&orientation);
    Ok(true)
}
fn process_exists(proc_name: &str) -> bool {
    !String::from_utf8(
        Command::new("pidof")
            .arg(proc_name)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .is_empty()
}
