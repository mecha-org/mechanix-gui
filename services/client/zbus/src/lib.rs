mod proxies;

pub mod wireless {
    use crate::proxies;
    pub use mechanix_zbus_services::{
        KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse,
        WirelessScanListResponse,
    };
    pub use proxies::wireless_proxy::WirelessService;
}

pub mod power {
    use crate::proxies;
    pub use proxies::power_proxy::Power;
}

pub mod bluetooth {
    use crate::proxies;
    pub use proxies::bluetooth_proxy::BluetoothService;
}

pub mod host_metrics {
    use crate::proxies;
    pub use mechanix_zbus_services::MemoryInfoResponse;
    pub use proxies::host_metrics::HostMetrics;
}

pub mod display {
    use crate::proxies;
    pub use proxies::display_proxy::Display;
}
