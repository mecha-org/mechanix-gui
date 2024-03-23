use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct RunningAppsService {}

impl RunningAppsService {
    pub async fn get_running_apps_status() -> Result<i32> {
        let task = "get_running_apps_status";

        let state = vec![5, 10, 20];

        let current_state = *state.get((Local::now().second() % 4) as usize).unwrap();

        Ok(current_state)
    }
}
