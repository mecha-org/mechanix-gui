use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::host_metrics::{HostMetrics, MemoryInfoResponse, NotificationStream};
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

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = HostMetrics::get_notification_stream().await?;
        Ok(stream)
    }
}
