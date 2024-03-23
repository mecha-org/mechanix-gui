use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct CpuService {}

impl CpuService {
    pub async fn get_cpu_usage() -> Result<i32> {
        let task = "get_cpu_usage";

        let state = vec![5, 10, 20];

        let current_state = *state.get((Local::now().second() % 4) as usize).unwrap();

        Ok(current_state)
    }
}
