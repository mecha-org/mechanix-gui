pub mod interfaces;
pub use interfaces as system_interfaces;
pub use system_interfaces::{
    //bluetooth interface
    bluetooth_event_notification_stream, BluetoothBusInterface, BluetoothNotificationEvent,

    //wireless interface
    KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse,
    WirelessNotificationEvent, WirelessScanListResponse,
};
