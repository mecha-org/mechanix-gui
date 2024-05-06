use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection, SignalContext,
};

use tokio::time::{self, Duration};

use mechanix_bluetooth_ctl::Bluetooth;

#[derive(Clone, Copy)]
pub struct BluetoothBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanResultResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BluetoothScanResponse {
    pub address: String,
    pub address_type: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub class: Option<u32>,
    pub rssi: Option<i16>,
    pub tx_power: Option<i16>,
    pub is_paired: bool,
    pub is_trusted: bool,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanResultResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BluetoothScanListResponse {
    pub bluetooth_devices: Vec<BluetoothScanResponse>,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `BluetoothAdapterInfoResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BluetoothAdapterInfoResponse {
    pub name: String,
    pub address: String,
    pub address_type: String,
    pub system_name: String,
    pub friendly_name: String,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
    pub active_advertising_instances: u8,
    pub supported_advertising_instances: u8,
    pub supported_advertising_includes: Vec<String>,
    pub supported_advertising_features: Vec<String>,
    pub max_advertisement_length: u8,
    pub max_scan_response_length: u8,
    pub min_tx_power: i16,
    pub max_tx_power: i16,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `BluetoothAdapterInfoResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BluetoothAdapterInfoListResponse {
    pub bluetooth_adapter_info: Vec<BluetoothAdapterInfoResponse>,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `BluetoothNotificationEvent` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BluetoothNotificationEvent {
    pub is_connected: bool,
    pub is_enabled: bool,
}

#[interface(name = "org.mechanix.services.Bluetooth")]
impl BluetoothBusInterface {
    pub async fn status(&self) -> Result<i8, ZbusError> {
        let bluetooth = Bluetooth::new();
        let result = match bluetooth.status().await {
            Ok(status) => status,
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get bluetooth status".to_string(),
                ));
            }
        };

        Ok(result)
    }

    pub async fn is_connected(&self) -> Result<i8, ZbusError> {
        let bluetooth = Bluetooth::new();

        let result = match bluetooth.is_connected().await {
            Ok(status) => status,
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get bluetooth connection status".to_string(),
                ));
            }
        };

        let result = if result { 1 } else { 0 };

        Ok(result)
    }

    pub async fn enable(&self) -> Result<(), ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.enable().await {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(ZbusError::Failed("Failed to enable bluetooth".to_string()));
            }
        }
    }

    pub async fn disable(&self) -> Result<(), ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.disable().await {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(ZbusError::Failed("Failed to disable bluetooth".to_string()));
            }
        }
    }

    pub async fn scan(&self) -> Result<BluetoothScanListResponse, ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.scan().await {
            Ok(res) => Ok(BluetoothScanListResponse {
                bluetooth_devices: res
                    .iter()
                    .map(|x| BluetoothScanResponse {
                        address: x.address.to_string(),
                        address_type: x.address_type.to_string(),
                        name: x.name.clone(),
                        icon: x.icon.clone(),
                        class: x.class,
                        rssi: x.rssi,
                        tx_power: x.tx_power,
                        is_paired: x.is_paired,
                        is_trusted: x.is_trusted,
                    })
                    .collect::<Vec<BluetoothScanResponse>>(),
            }),
            Err(_) => {
                println!("Failed to scan for bluetooth devices");
                return Err(ZbusError::Failed(
                    "Failed to scan for bluetooth devices".to_string(),
                ));
            }
        }
    }

    pub async fn connect(&self, address: &str) -> Result<(), ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.add_bluetooth(address).await {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to connect to bluetooth device".to_string(),
                ));
            }
        }
    }

    pub async fn disconnect(&self, address: &str) -> Result<(), ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.remove_device(address).await {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to disconnect from bluetooth device".to_string(),
                ));
            }
        }
    }

    #[zbus(signal)]
    async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: BluetoothNotificationEvent,
    ) -> Result<(), zbus::Error>;

    pub async fn send_notification_stream(&self) -> Result<(), ZbusError> {
        let mut interval = time::interval(Duration::from_secs(15));
        let mut previous_is_enable: Option<bool> = None;
        let mut previous_is_connected: Option<bool> = None;

        loop {
            interval.tick().await;
            let bluetooth = Bluetooth::new();

            // Check Bluetooth power status
            let is_enable = match bluetooth.status().await {
                Ok(status) => status == 1,
                Err(e) => {
                    previous_is_enable.unwrap_or(false) // Use previous value or default to false
                }
            };

            // Check Bluetooth connection status
            let is_connected = bluetooth.is_connected().await.unwrap_or_else(|e| {
                previous_is_connected.unwrap_or(false) // Use previous value or default to false
            });

            // Send signal if there's a change in status
            if previous_is_enable != Some(is_enable) || previous_is_connected != Some(is_connected)
            {
                let ctxt = SignalContext::new(
                    &Connection::system().await?,
                    "/org/mechanix/services/Bluetooth",
                )?;
                self.notification(
                    &ctxt,
                    BluetoothNotificationEvent {
                        is_connected: is_connected,
                        is_enabled: is_enable,
                    },
                )
                .await?;
                previous_is_enable = Some(is_enable);
                previous_is_connected = Some(is_connected);
            }
        }
    }

    pub async fn get_bluetooth_properties(
        &self,
    ) -> Result<BluetoothAdapterInfoListResponse, ZbusError> {
        let bluetooth = Bluetooth::new();
        match bluetooth.get_adapter_info().await {
            Ok(res) => Ok(BluetoothAdapterInfoListResponse {
                bluetooth_adapter_info: res
                    .iter()
                    .map(|x| BluetoothAdapterInfoResponse {
                        name: x.name.clone(),
                        address: x.address.clone(),
                        address_type: x.address_type.clone(),
                        system_name: x.system_name.clone(),
                        friendly_name: x.friendly_name.clone(),
                        powered: x.powered,
                        discoverable: x.discoverable,
                        pairable: x.pairable,
                        active_advertising_instances: x.active_advertising_instances,
                        supported_advertising_instances: x.supported_advertising_instances,
                        supported_advertising_includes: x.supported_advertising_includes.clone(),
                        supported_advertising_features: x.supported_advertising_features.clone(),
                        max_advertisement_length: x.max_advertisement_length,
                        max_scan_response_length: x.max_scan_response_length,
                        min_tx_power: x.min_tx_power,
                        max_tx_power: x.max_tx_power,
                    })
                    .collect::<Vec<BluetoothAdapterInfoResponse>>(),
            }),
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get bluetooth properties".to_string(),
                ));
            }
        }
    }
}
