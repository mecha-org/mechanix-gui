use serde::{Deserialize, Serialize};
use mechanix_system_dbus_server::system_interfaces::{
    KnownNetworkListResponse, WirelessInfoResponse, WirelessNotificationEvent,
    WirelessScanListResponse,
};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result, SignalContext};

#[proxy(
    interface = "org.mechanix.services.Wireless",
    default_service = "org.mechanix.services.Wireless",
    default_path = "/org/mechanix/services/Wireless"
)]
trait Wireless {
    async fn scan(&self) -> Result<WirelessScanListResponse>;
    async fn known_networks(&self) -> Result<KnownNetworkListResponse>;
    async fn select_network(&self, network_id: &str) -> Result<()>;

    async fn info(&self) -> Result<WirelessInfoResponse>;
    async fn status(&self) -> Result<bool>;
    async fn enable(&self) -> Result<bool>;
    async fn disable(&self) -> Result<bool>;
    async fn connect(&self, ssid: &str, password: &str) -> Result<()>;

    async fn disconnect(&self, ssid: &str) -> Result<()>;
    #[zbus(signal)]
    async fn notification(&self, event: WirelessNotificationEvent) -> Result<()>;
}

pub struct WirelessService;

impl WirelessService {
    pub async fn scan() -> Result<WirelessScanListResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.scan().await?;
        Ok(reply)
    }

    pub async fn known_networks() -> Result<KnownNetworkListResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.known_networks().await?;
        Ok(reply)
    }

    pub async fn info() -> Result<WirelessInfoResponse> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.info().await?;
        Ok(reply)
    }

    pub async fn wireless_status() -> Result<bool> {
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply = proxy.status().await?;
        Ok(reply)
    }

    pub async fn enable_wireless() -> Result<bool> {
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply = proxy.enable().await?;
        Ok(reply)
    }

    pub async fn disable_wireless() -> Result<bool> {
        let connection = Connection::system().await?;

        let proxy = WirelessProxy::new(&connection).await?;

        let reply = proxy.disable().await?;
        Ok(reply)
    }

    pub async fn connect_to_network(ssid: &str, password: &str) -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.connect(ssid, password).await?;
        Ok(())
    }

    pub async fn connect_to_known_network(network_id: &str) -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.select_network(network_id).await?;
        Ok(())
    }

    pub async fn disconnect(value: &str) -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let reply = proxy.disconnect(value).await?;
        Ok(reply)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = WirelessProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
