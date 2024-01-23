use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use command::execute_command;
use glob::glob;
use tokio::sync::mpsc;
use tracing::{error, info};
use wayland_client::protocol::wl_output::Transform;
#[derive(Debug, Clone)]
pub struct Orientation {
    pub vector: (f32, f32),
    pub wayland_state: Transform,
    pub x_state: &'static str,
    pub matrix: [&'static str; 9],
}

use crate::{
    backends::{sway::SwayBackend, wlroots::WaylandBackend, xorg::XorgBackend, DisplayManager},
    handlers::{
        rotation::errors::{RotationHandlerError, RotationHandlerErrorCodes},
        transform::handler::WlrootsHandlerMessage,
    },
    settings::{update_settings, RotationConfigs, RotationSettings},
};

pub struct DispatchRotationParams {
    pub wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>,
    pub rotation_configs: RotationConfigs,
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
    let rotation_configs = dispatch_rotation_params.rotation_configs;

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

    //get previous orientation
    let old_state = match backend.get_rotation_state() {
        Ok(v) => v,
        Err(e) => {
            bail!(RotationHandlerError::new(
                RotationHandlerErrorCodes::GetOldTransformStateError,
                format!("unable to get old transform state: {}", e),
                true
            ))
            // Transform::Normal
        }
    };

    //get current orientation
    let current_orientation = match get_current_orientation(backend, rotation_configs.clone()) {
        Ok(v) => v,
        Err(e) => {
            bail!(RotationHandlerError::new(
                RotationHandlerErrorCodes::GetCurrentOrientationError,
                format!("Unable to get current orientation state: {}", e),
                true
            ))
            // Orientation {
            //     vector: (0.0, 1.0),
            //     wayland_state: Transform::_180,
            //     x_state: "inverted",
            //     matrix: ["-1", "0", "1", "0", "-1", "1", "0", "0", "1"],
            // }
        }
    };

    info!(
        "current_orientation is {:?} {:?}",
        old_state, current_orientation,
    );

    //compare orientations
    //based on this send events to wlroots
    if old_state != current_orientation.wayland_state {
        let _ = dispatch_rotation_params
            .wlroots_sender_tx
            .send(WlrootsHandlerMessage::Rotate {
                output_name: rotation_configs.display.clone(),
                transform: current_orientation.wayland_state,
            })
            .await;
    }

    Ok(true)
}

pub fn get_current_orientation(
    backend: Box<dyn DisplayManager>,
    config: RotationConfigs,
) -> Result<Orientation> {
    let mut path_x: String = "".to_string();
    let mut path_y: String = "".to_string();
    let mut path_z: String = "".to_string();

    for entry in glob("/sys/bus/iio/devices/iio:device*/in_accel_*_raw").unwrap() {
        match entry {
            Ok(path) => {
                if path.to_str().unwrap().contains("x_raw") {
                    path_x = path.to_str().unwrap().to_owned();
                } else if path.to_str().unwrap().contains("y_raw") {
                    path_y = path.to_str().unwrap().to_owned();
                } else if path.to_str().unwrap().contains("z_raw") {
                    path_z = path.to_str().unwrap().to_owned();
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if !Path::new(&path_x).exists() && !Path::new(&path_y).exists() && !Path::new(&path_z).exists()
    {
        bail!(RotationHandlerError::new(
            RotationHandlerErrorCodes::UnknownAccelerometerDeviceError,
            format!("Unknown Accelerometer Device"),
            true
        ));
    }

    let orientations = [
        Orientation {
            vector: (0.0, -1.0),
            wayland_state: Transform::Normal,
            x_state: "normal",
            matrix: ["1", "0", "0", "0", "1", "0", "0", "0", "1"],
        },
        Orientation {
            vector: (0.0, 1.0),
            wayland_state: Transform::_180,
            x_state: "inverted",
            matrix: ["-1", "0", "1", "0", "-1", "1", "0", "0", "1"],
        },
        Orientation {
            vector: (-1.0, 0.0),
            wayland_state: Transform::_270,
            x_state: "right",
            matrix: ["0", "1", "0", "-1", "0", "1", "0", "0", "1"],
        },
        Orientation {
            vector: (1.0, 0.0),
            wayland_state: Transform::_90,
            x_state: "left",
            matrix: ["0", "-1", "1", "1", "0", "0", "0", "0", "1"],
        },
    ];

    let mut current_orient: &Orientation = &orientations[0];

    //-1996, 38, 797
    //1992, 26, 731
    //101, -1739, 1035 Inverted
    //-64, 2028, 450 Normal

    let x_raw = fs::read_to_string(path_x.as_str()).unwrap();
    let y_raw = fs::read_to_string(path_y.as_str()).unwrap();
    let z_raw = fs::read_to_string(path_z.as_str()).unwrap();
    let x_clean = x_raw.trim_end_matches('\n').parse::<f32>().unwrap_or(0.);
    let y_clean = y_raw.trim_end_matches('\n').parse::<f32>().unwrap_or(0.);
    let z_clean = z_raw.trim_end_matches('\n').parse::<f32>().unwrap_or(0.);

    // Normalize vectors
    let norm_factor = &config
        .normalization_factor
        .unwrap_or_else(|| f32::sqrt(x_clean * x_clean + y_clean * y_clean + z_clean * z_clean));

    let mut mut_x: f32 = x_clean / norm_factor;
    let mut mut_y: f32 = y_clean / norm_factor;
    let mut mut_z: f32 = z_clean / norm_factor;

    // Apply inversions
    if config.invert_x {
        mut_x = -mut_x;
    }
    if config.invert_y {
        mut_y = -mut_y;
    }
    if config.invert_z {
        mut_z = -mut_z;
    }
    // Switch axes as requested
    // let x = match x_source {
    //     'y' => mut_y,
    //     'z' => mut_z,
    //     _ => mut_x,
    // };
    // let y = match y_source {
    //     'x' => mut_x,
    //     'z' => mut_z,
    //     _ => mut_y,
    // };

    let x = mut_x;
    let y = mut_y;

    for orient in orientations.iter() {
        let d = (x - orient.vector.0).powf(2.0) + (y - orient.vector.1).powf(2.0);
        if d < config.threshold {
            current_orient = orient;
            break;
        }
    }

    Ok(current_orient.clone())
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

// if current_orient.wayland_state != old_state {
//     let old_env = transform_to_env(&old_state);
//     let new_env = transform_to_env(&current_orient.wayland_state);

//     backend.change_rotation_state(current_orient);

//     old_state = current_orient.wayland_state;
// }
