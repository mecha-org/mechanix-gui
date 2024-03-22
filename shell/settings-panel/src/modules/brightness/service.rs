use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct BrightnessService {}

impl BrightnessService {
    pub async fn get_brightness_value() -> Result<i32> {
        let task = "get_brightness_value";

        let state = vec![5, 10, 20];

        let current_state = *state.get((Local::now().second() % 4) as usize).unwrap();

        Ok(current_state)
    }
}
