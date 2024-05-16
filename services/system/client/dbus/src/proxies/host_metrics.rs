use system_dbus_server::system_interfaces::{HostMetricsNotificationEvent, MemoryInfoResponse};
use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::Type, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.HostMetrics",
    default_service = "org.mechanix.services.HostMetrics",
    default_path = "/org/mechanix/services/HostMetrics"
)]
trait HostMetricsBusInterface {
    async fn get_cpu_usage(&self) -> Result<f32>;
    async fn get_memory_info(&self) -> Result<MemoryInfoResponse>;
    #[zbus(signal)]
    async fn notification(&self, event: HostMetricsNotificationEvent) -> Result<()>;
}

pub struct HostMetrics;

impl HostMetrics {
    pub async fn get_cpu_usage() -> Result<f32> {
        let connection = Connection::system().await?;
        let proxy = HostMetricsBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_cpu_usage().await?;
        Ok(reply)
    }

    pub async fn get_memory_info() -> Result<MemoryInfoResponse> {
        let connection = Connection::system().await?;
        let proxy = HostMetricsBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_memory_info().await?;
        Ok(reply)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = HostMetricsBusInterfaceProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
