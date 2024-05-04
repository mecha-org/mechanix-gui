use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::host_metrics::{HostMetrics, NotificationStream};
use tracing::{debug, error, info};

use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};

pub struct CpuService {}

impl CpuService {
    pub async fn get_cpu_usage() -> Result<f32> {
        let task = "get_cpu_usage";

        let cpu_usage = match HostMetrics::get_cpu_usage().await {
            Ok(v) => {
                //println!("CpuService::get_cpu_usage() {}", v);
                v
            }
            Err(e) => {
                println!("error while getting cpu usage {:?}", e.to_string());
                bail!(SettingsPanelError::new(
                    SettingsPanelErrorCodes::GetCpuUsageError,
                    e.to_string(),
                ))
            }
        };

        Ok(cpu_usage)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = HostMetrics::get_notification_stream().await?;
        Ok(stream)
    }
}
