use futures::{pin_mut, stream::SelectAll, FutureExt, StreamExt, TryFutureExt};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
    time::Duration,
};

use anyhow::Result;

use bluer::{
    Adapter, AdapterEvent, Address, AddressType, Device, DeviceEvent, DeviceProperty, Session, Uuid,
};
use tokio::{select, time::sleep};

#[derive(Clone, Copy)]
pub struct UuidOrShort(pub Uuid);

pub struct Bluetooth {
    // Address of local Bluetooth adapter
    bind: Option<Address>,
    address: Vec<Address>,
    timeout: u64,
    public_only: bool,
}

#[derive(Debug)]
pub struct BluetoothAdapterInfo {
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

impl Display for BluetoothAdapterInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Bluetooth adapter {}\n\
             Address: {} [{}]\n\
             System name: {}\n\
             Friendly name: {}\n\
             Powered: {}\n\
             Discoverable: {}\n\
             Pairable: {}\n\
             Advertising:\n\
             - Active instances: {}\n\
             - Supported instances: {}\n\
             - Supported includes: {}\n\
             - Supported features: {}\n\
             - Max. advertisement: {} bytes\n\
             - Max. scan response: {} bytes\n\
             - Min. TX power: {} dBm\n\
             - Max. TX power: {} dBm",
            self.name,
            self.address,
            self.address_type,
            self.system_name,
            self.friendly_name,
            self.powered,
            self.discoverable,
            self.pairable,
            self.active_advertising_instances,
            self.supported_advertising_instances,
            self.supported_advertising_includes.join(", "),
            self.supported_advertising_features.join(", "),
            self.max_advertisement_length,
            self.max_scan_response_length,
            self.min_tx_power,
            self.max_tx_power
        )
    }
}

type BlurResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug)]
pub struct BluetoothDeviceInfo {
    pub address: Address,
    pub address_type: AddressType,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub class: Option<u32>,
    pub rssi: Option<i16>,
    pub tx_power: Option<i16>,
    pub is_paired: bool,
    pub is_trusted: bool,
    pub uuids: HashSet<bluer::Uuid>,
    pub service_data: HashMap<bluer::Uuid, Vec<u8>>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
}

async fn get_device_info(dev: &Device) -> Result<BluetoothDeviceInfo> {
    let address = dev.address();
    let address_type = dev.address_type().await.unwrap_or_default();
    let name = dev.name().await?;
    let icon = dev.icon().await?;
    let class = dev.class().await?;
    let rssi = dev.rssi().await?;
    let tx_power = dev.tx_power().await?;
    let is_paired = dev.is_paired().await?;
    let is_trusted = dev.is_trusted().await?;
    let uuids = dev.uuids().await?.unwrap_or_default();
    let service_data = dev.service_data().await?.unwrap_or_default();
    let manufacturer_data = dev.manufacturer_data().await?.unwrap_or_default();

    Ok(BluetoothDeviceInfo {
        address,
        address_type,
        name,
        icon,
        class,
        rssi,
        tx_power,
        is_paired,
        is_trusted,
        uuids,
        service_data,
        manufacturer_data,
    })
}

impl Bluetooth {
    pub fn new() -> Bluetooth {
        Bluetooth {
            bind: None,
            address: Vec::new(),
            timeout: 5,
            public_only: true,
        }
    }

    pub async fn get_adapter_info(&self) -> Result<Vec<BluetoothAdapterInfo>> {
        let session = Session::new().await?;
        let adapter_names = session.adapter_names().await?;
        let mut adapter_infos = Vec::new();

        for adapter_name in adapter_names {
            let adapter = session.adapter(&adapter_name)?;
            let info = BluetoothAdapterInfo {
                name: adapter_name,
                address: adapter.address().await?.to_string(),
                address_type: adapter.address_type().await?.to_string(),
                system_name: adapter.system_name().await?,
                friendly_name: adapter.alias().await?,
                powered: adapter.is_powered().await?,
                discoverable: adapter.is_discoverable().await?,
                pairable: adapter.is_pairable().await?,
                active_advertising_instances: adapter.active_advertising_instances().await?.into(),
                supported_advertising_instances: adapter
                    .supported_advertising_instances()
                    .await?
                    .into(),
                supported_advertising_includes: adapter
                    .supported_advertising_system_includes()
                    .await?
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect(),
                supported_advertising_features: adapter
                    .supported_advertising_features()
                    .await?
                    .unwrap_or_default()
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect(),
                max_advertisement_length: adapter
                    .supported_advertising_capabilities()
                    .await?
                    .map(|c| c.max_advertisement_length)
                    .unwrap_or_default(),
                max_scan_response_length: adapter
                    .supported_advertising_capabilities()
                    .await?
                    .map(|c| c.max_scan_response_length)
                    .unwrap_or_default(),
                min_tx_power: adapter
                    .supported_advertising_capabilities()
                    .await?
                    .map(|c| c.min_tx_power)
                    .unwrap_or_default(),
                max_tx_power: adapter
                    .supported_advertising_capabilities()
                    .await?
                    .map(|c| c.max_tx_power)
                    .unwrap_or_default(),
            };
            adapter_infos.push(info);
        }

        Ok(adapter_infos)
    }

    pub async fn status(&self) -> BlurResult<i8> {
        let (_session, adapter) = get_session_adapter(self.bind).await?;
        let status = if adapter.is_powered().await? { 1 } else { 0 };
        Ok(status)
    }

    pub async fn enable(&self) -> BlurResult<()> {
        let (_session, adapter) = get_session_adapter(self.bind).await?;
        adapter.set_powered(true).await?;
        Ok(())
    }

    pub async fn disable(&self) -> BlurResult<()> {
        let (_session, adapter) = get_session_adapter(self.bind).await?;
        adapter.set_powered(false).await?;
        Ok(())
    }

    pub async fn scan(&self) -> BlurResult<Vec<BluetoothDeviceInfo>> {
        let (_session, adapter) = get_session_adapter(self.bind).await?.into();
        let mut discover = adapter.discover_devices().await?;
        let mut changes = SelectAll::new();
        let mut timeout = sleep(Duration::from_secs(self.timeout)).boxed();

        let mut addresses: HashSet<_> = self.address.clone().drain(..).collect();
        let mut done = HashSet::new();
        let filter = !addresses.is_empty();

        let mut discovered_devices = Vec::new();
        loop {
            if filter && addresses.is_empty() {
                break;
            }
            let addr = select! {
                _ = &mut timeout => break,
                evt = discover.next() => {
                    match evt {
                        Some(AdapterEvent::DeviceAdded(addr)) => addr,
                        None => break,
                        _ => continue,
                    }
                },
                Some((addr, evt)) = changes.next() => {
                    match evt {
                        DeviceEvent::PropertyChanged(DeviceProperty::Rssi(_)) => addr,
                        _ => continue,
                    }
                }
            };
            if (filter && !addresses.contains(&addr)) || done.contains(&addr) {
                continue;
            }

            let dev = adapter.device(addr)?;
            if self.public_only
                && dev.address_type().await.unwrap_or_default() == AddressType::LeRandom
            {
                continue;
            }
            if let Ok(Some(_)) = dev.rssi().await {
                // If RSSI is available, device is present.
                let device_info = get_device_info(&dev).await?;
                discovered_devices.push(device_info);
                let _ = dev.disconnect().await;
                println!();
                addresses.remove(&addr);
                done.insert(addr);
            } else {
                // Device may be cached, wait for RSSI to become available.
                if let Ok(events) = dev.events().await {
                    changes.push(events.map(move |evt| (addr, evt)).boxed());
                }
            }

            print!("Timeout in {} seconds\r", self.timeout);

            timeout = sleep(Duration::from_secs(self.timeout)).boxed();
        }

        Ok(discovered_devices)
    }

    pub async fn add_bluetooth(&self, address: &str) -> BlurResult<()> {
        //convert address to blur::Address
        let address = Address::from_str(address)?;
        let (_session, adapter) = get_session_adapter(self.bind).await?;
        let dev = find_device(&adapter, address).await?;
        connect(&dev).await?;
        Ok(())
    }

    pub async fn remove_device(&self, address: &str) -> BlurResult<()> {
        let (_session, adapter) = get_session_adapter(self.bind).await?;
        let addr = Address::from_str(address)?;
        let dev = adapter.device(addr)?;
        dev.disconnect().await?;
        Ok(())
    }
}

async fn get_session_adapter(addr: Option<Address>) -> BlurResult<(Session, Adapter)> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;

    match addr {
        Some(addr) => {
            for adapter_name in adapter_names {
                let adapter = session.adapter(&adapter_name)?;
                if adapter.address().await? == addr {
                    adapter.set_powered(true).await?;
                    return Ok((session, adapter));
                }
            }
            Err("specified Bluetooth adapter not present".into())
        }
        None => {
            let adapter_name = adapter_names
                .first()
                .ok_or("no Bluetooth adapter present")?;
            let adapter = session.adapter(adapter_name)?;
            adapter.set_powered(true).await?;
            Ok((session, adapter))
        }
    }
}

async fn find_device(adapter: &Adapter, address: Address) -> BlurResult<Device> {
    let mut disco = adapter.discover_devices().await?;
    let timeout = sleep(Duration::from_secs(20));
    pin_mut!(timeout);

    loop {
        select! {
            Some(evt) = disco.next() => {
                if let AdapterEvent::DeviceAdded(addr) = evt {
                    if addr == address {
                        return Ok(adapter.device(addr)?);
                    }
                }
            }
            _ = &mut timeout => {
                return Err("device not found".into());
            }
        }
    }
}

async fn connect(device: &Device) -> BlurResult<()> {
    if !device.is_connected().await? {
        let mut retries = 2;
        loop {
            match device.connect().and_then(|_| device.services()).await {
                Ok(_) => break,
                Err(_) if retries > 0 => {
                    retries -= 1;
                }
                Err(err) => return Err(err.into()),
            }
        }
    }
    Ok(())
}
