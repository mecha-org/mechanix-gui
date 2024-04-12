mod proxies;

pub mod wireless {
    use crate::proxies;
    pub use mechanix_zbus_server::{
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
