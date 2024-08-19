use core::fmt;

use crate::errors::OnScreenDisplayError;
use crate::errors::OnScreenDisplayErrorCodes::{GetBatteryError, GetBatteryStatusError};
use crate::AppMessage;
use anyhow::{bail, Result};
use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use tracing::{error, info};
use upower::BatteryStatus;

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_level() -> Result<(u8, BatteryStatus)> {
        let task = "get_battery_level";

        let battery = match upower::get_battery().await {
            Ok(battery) => battery,
            Err(e) => bail!(OnScreenDisplayError::new(GetBatteryError, e.to_string(),)),
        };

        let percentage = match battery.percentage().await {
            Ok(p) => p,
            Err(e) => bail!(OnScreenDisplayError::new(
                GetBatteryStatusError,
                e.to_string(),
            )),
        };

        let state = match battery.state().await {
            Ok(s) => s,
            Err(e) => bail!(OnScreenDisplayError::new(
                GetBatteryStatusError,
                e.to_string(),
            )),
        };

        let battery_status = BatteryStatus::try_from(state).unwrap();

        Ok((percentage as u8, battery_status))
    }
}

pub struct BatteryServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BatteryServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        match BatteryService::get_battery_level().await {
            Ok((capacity, status)) => {
                let _ = self.app_channel.send(AppMessage::Battery {
                    level: capacity,
                    status,
                });
            }
            Err(e) => {
                error!("error while getting battery level {}", e);
                let _ = self.app_channel.send(AppMessage::Battery {
                    level: 0,
                    status: BatteryStatus::Unknown,
                });
            }
        };

        let battery_r = upower::get_battery().await;

        if let Err(e) = battery_r {
            println!("Error while getting battery {:?}", e);
            return;
        }

        let battery = battery_r.unwrap();

        let mut state_stream = battery.receive_state_changed().await;
        let app_channel = self.app_channel.clone();
        let battery_cloned = battery.clone();

        let state_stream_t = tokio::spawn(async move {
            while let Some(msg) = state_stream.next().await {
                if let Ok(state) = msg.get().await {
                    let status = BatteryStatus::try_from(state).unwrap();
                    let level = battery_cloned.percentage().await.unwrap() as u8;
                    let _ = app_channel.send(AppMessage::Battery { level, status });
                };
            }
        });

        let mut percentage_stream = battery.receive_percentage_changed().await;
        let app_channel = self.app_channel.clone();
        let percentage_stream_t = tokio::spawn(async move {
            while let Some(msg) = percentage_stream.next().await {
                if let Ok(percentage) = msg.get().await {
                    let status = BatteryStatus::try_from(battery.state().await.unwrap()).unwrap();
                    let level = percentage as u8;
                    let _ = app_channel.send(AppMessage::Battery { level, status });
                };
            }
        });

        state_stream_t.await.unwrap();
        percentage_stream_t.await.unwrap();
    }
}

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum BatteryServiceErrorCodes {
    #[default]
    UnknownError,
    GetBatteryInfoError,
}

impl fmt::Display for BatteryServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BatteryServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            BatteryServiceErrorCodes::GetBatteryInfoError => write!(f, "GetBatteryInfoError"),
        }
    }
}

/// # BatteryServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`BatteryServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BatteryServiceError {
    pub code: BatteryServiceErrorCodes,
    pub message: String,
}

impl BatteryServiceError {
    pub fn new(code: BatteryServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BatteryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
