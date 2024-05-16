mod proxies;

pub mod wireless {
    use crate::proxies;
    pub use mechanix_system_dbus_server::system_interfaces::{
        KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse,
        WirelessScanListResponse,
    };
    pub use proxies::wireless_proxy::{NotificationStream, WirelessService};
}

pub mod power {
    use crate::proxies;
    pub use proxies::power_proxy::{NotificationStream, Power};
}

pub mod bluetooth {
    use crate::proxies;
    pub use proxies::bluetooth_proxy::{BluetoothService, NotificationStream};
}

pub mod host_metrics {
    use crate::proxies;
    pub use mechanix_system_dbus_server::system_interfaces::MemoryInfoResponse;
    pub use proxies::host_metrics::{HostMetrics, NotificationStream};
}

pub mod display {
    use crate::proxies;
    pub use proxies::display_proxy::{Display, NotificationStream};
}
