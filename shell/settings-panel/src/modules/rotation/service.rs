use crate::modules::rotation::errors::{RotationServiceError, RotationServiceErrorCodes};
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

use super::component::RotationStatus;

pub struct RotationService {}

impl RotationService {
    pub async fn get_rotation_status() -> Result<RotationStatus> {
        let task = "get_rotation_status";

        let rotation_state = vec![RotationStatus::Portrait, RotationStatus::Landscape];

        let current_state = *rotation_state
            .get((Local::now().second() % 4) as usize)
            .unwrap();

        Ok(current_state)
    }
}
