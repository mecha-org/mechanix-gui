use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct SoundService {}

impl SoundService {
    pub async fn get_sound_value() -> Result<i32> {
        let task = "get_sound_value";

        let state = vec![5, 5, 5, 5];

        let current_state = *state.get((Local::now().second() % 4) as usize).unwrap();

        Ok(current_state)
    }
    pub async fn set_sound_value(value: i32) -> Result<bool> {
        let task = "get_sound_value";

        Ok(true)
    }
}
