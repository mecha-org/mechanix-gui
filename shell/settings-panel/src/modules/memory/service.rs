use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

pub struct MemoryService {}

impl MemoryService {
    pub async fn get_memory_usage() -> Result<i32> {
        let task = "get_memory_usage";

        let state = vec![5, 10, 20];

        let current_state = *state.get((Local::now().second() % 4) as usize).unwrap();

        Ok(current_state)
    }
}
