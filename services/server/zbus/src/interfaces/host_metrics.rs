use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};

use tokio::time::{self, Duration};

use mechanix_host_metrics::HostMetrics;

#[derive(Clone)]
pub struct HostMetricsBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, PartialEq)]
// `Type` treats `HostMetricsResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct MemoryInfoResponse {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, PartialEq)]
// `Type` treats `HostMetricsResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct DiskInfoResponse {
    pub available_space: u64,
    pub total_space: u64,
    pub used_space: u64,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, PartialEq)]
// `Type` treats `HostMetricsResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct LoadAverageResponse {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `HostMetricsResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct NetworkDataInfo {
    pub interface_name: String,
    pub mac_address: String,
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Debug, DeserializeDict, SerializeDict, Type)]
// `Type` treats `HostMetricsNotificationEvents` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct HostMetricsNotificationEvent {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub available_memory: u64,
}

#[interface(name = "org.mechanix.services.HostMetrics")]
impl HostMetricsBusInterface {
    pub async fn get_cpu_usage(&self) -> Result<f32, ZbusError> {
        let host_metrics = HostMetrics::new();
        let cpu_usage = host_metrics.cpu_usage();
        Ok(cpu_usage)
    }

    pub async fn get_memory_usage(&self) -> Result<u64, ZbusError> {
        let host_metrics = HostMetrics::new();
        let memory_usage = host_metrics.memory_usage();
        Ok(memory_usage)
    }

    pub async fn get_disk_info(&self) -> Result<DiskInfoResponse, ZbusError> {
        let host_metrics = HostMetrics::new();
        let disk_usage = host_metrics.disk_info();

        let disk_info = DiskInfoResponse {
            available_space: disk_usage.available_space,
            total_space: disk_usage.total_space,
            used_space: disk_usage.used_space,
        };

        Ok(disk_info)
    }

    pub async fn get_network_usage(&self) -> Result<Vec<NetworkDataInfo>, ZbusError> {
        let host_metrics = HostMetrics::new();
        let network_usage = host_metrics.network_usage();

        // convert network_usage to NetworkDataInfo
        let network_usage: Vec<NetworkDataInfo> = network_usage
            .iter()
            .map(|data| NetworkDataInfo {
                interface_name: data.interface_name.clone(),
                mac_address: data.mac_address.to_string(),
                received: data.received,
                transmitted: data.transmitted,
            })
            .collect();

        Ok(network_usage)
    }

    //cpu frequency
    pub async fn get_cpu_freq(&self) -> Result<u64, ZbusError> {
        let host_metrics = HostMetrics::new();
        let cpu_freq = host_metrics.cpu_freq();

        Ok(cpu_freq)
    }

    //memory info
    pub async fn get_memory_info(&self) -> Result<MemoryInfoResponse, ZbusError> {
        let host_metrics = HostMetrics::new();
        let memory = host_metrics.memory_info();

        let memory_info = MemoryInfoResponse {
            total_memory: memory.total_memory,
            used_memory: memory.used_memory,
            total_swap: memory.total_swap,
            used_swap: memory.used_swap,
        };

        Ok(memory_info)
    }

    // uptime
    pub async fn get_uptime(&self) -> Result<u64, ZbusError> {
        let host_metrics = HostMetrics::new();
        let uptime = host_metrics.uptime();

        Ok(uptime)
    }

    //load average
    pub async fn get_load_average(&self) -> Result<LoadAverageResponse, ZbusError> {
        let host_metrics = HostMetrics::new();
        let load_average = host_metrics.load_average();

        let load_average = LoadAverageResponse {
            one: load_average.one,
            five: load_average.five,
            fifteen: load_average.fifteen,
        };

        Ok(load_average)
    }

    //network usage
    pub async fn get_network_data(&self) -> Result<Vec<NetworkDataInfo>, ZbusError> {
        let host_metrics = HostMetrics::new();
        let network_data = host_metrics.network_usage();

        let mut network_data_info = Vec::new();
        for data in network_data {
            let network_data = NetworkDataInfo {
                interface_name: data.interface_name,
                mac_address: data.mac_address.to_string(),
                received: data.received,
                transmitted: data.transmitted,
            };
            network_data_info.push(network_data);
        }

        Ok(network_data_info)
    }

    // notification signal
    #[zbus(signal)]
    pub async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: HostMetricsNotificationEvent,
    ) -> Result<(), zbus::Error>;
}

#[allow(dead_code)]
pub async fn host_metrics_event_notification_stream(
    host_metrics_bus: &HostMetricsBusInterface,
    conn: &zbus::Connection,
) -> Result<(), ZbusError> {
    let mut interval = time::interval(Duration::from_secs(10));

    let mut previous_cpu_usage: Option<f32> = None;
    let mut previous_total_memory: Option<u64> = None;
    let mut previous_available_memory: Option<u64> = None;

    loop {
        interval.tick().await;
        let host_metrics = HostMetrics::new();

        let cpu_usage;
        let total_memory;
        let available_memory;

        cpu_usage = host_metrics.cpu_usage();
        total_memory = host_metrics.memory_info().total_memory;
        available_memory = host_metrics.memory_info().used_memory;

        // Check if there's a change in any of the values
        if previous_cpu_usage != Some(cpu_usage)
            || previous_total_memory != Some(total_memory)
            || previous_available_memory != Some(available_memory)
        {
            // Trigger the notification here

            let ctxt = SignalContext::new(conn, "/org/mechanix/services/HostMetrics")?;

            host_metrics_bus
                .notification(
                    &ctxt,
                    HostMetricsNotificationEvent {
                        cpu_usage,
                        total_memory,
                        available_memory,
                    },
                )
                .await?;

            // Update the previous values
            previous_cpu_usage = Some(cpu_usage);
            previous_total_memory = Some(total_memory);
            previous_available_memory = Some(available_memory);
        }
    }
}
