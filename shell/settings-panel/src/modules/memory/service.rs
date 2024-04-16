use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::host_metrics::{HostMetrics, MemoryInfoResponse};
use tracing::{debug, error, info};

use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};

pub struct MemoryService {}

impl MemoryService {
    pub async fn get_memory_usage() -> Result<MemoryInfoResponse> {
        let task = "get_memory_usage";

        let memory_info = match HostMetrics::get_memory_info().await {
            Ok(v) => {
                // println!("MemoryService::get_memory_usage() {:?}", v);
                v
            }
            Err(e) => {
                println!("error while getting memory usage {:?}", e.to_string());
                bail!(SettingsPanelError::new(
                    SettingsPanelErrorCodes::GetMemoryInfoError,
                    e.to_string(),
                ))
            }
        };
        Ok(memory_info)
    }
}
