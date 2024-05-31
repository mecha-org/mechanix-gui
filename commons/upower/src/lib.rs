pub mod device;
pub mod upower;
use anyhow::Result;
use device::DeviceProxy;
use upower::UPowerProxy;
use zbus::Connection;

#[derive(Debug, Clone, PartialEq)]
pub enum BatteryStatus {
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullCharged,
    PendingCharge,
    PendingDischarge,
}

impl TryFrom<u32> for BatteryStatus {
    type Error = ();
    fn try_from(value: u32) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            0 => Ok(BatteryStatus::Unknown),
            1 => Ok(BatteryStatus::Charging),
            2 => Ok(BatteryStatus::Discharging),
            3 => Ok(BatteryStatus::Empty),
            4 => Ok(BatteryStatus::FullCharged),
            5 => Ok(BatteryStatus::PendingCharge),
            6 => Ok(BatteryStatus::PendingDischarge),
            _ => Err(()),
        }
    }
}

pub enum DeviceType {
    Unknown,
    LinePower,
    Battery,
    Ups,
    Monitor,
    Mouse,
    Keyboard,
    Pda,
    Phone,
    MediaPlayer,
    Tablet,
    Computer,
    GamingInput,
    Pen,
    Touchpad,
    Modem,
    Network,
    Headset,
    Speakers,
    Headphones,
    Video,
    OtherAudio,
    RemoteControl,
    Printer,
    Scanner,
    Camera,
    Wearable,
    Toy,
    BluetoothGeneric,
}

impl Into<u32> for DeviceType {
    fn into(self) -> u32 {
        match self {
            DeviceType::Unknown => 0,
            DeviceType::LinePower => 1,
            DeviceType::Battery => 2,
            DeviceType::Ups => 3,
            DeviceType::Monitor => 4,
            DeviceType::Mouse => 5,
            DeviceType::Keyboard => 6,
            DeviceType::Pda => 7,
            DeviceType::Phone => 8,
            DeviceType::MediaPlayer => 9,
            DeviceType::Tablet => 10,
            DeviceType::Computer => 11,
            DeviceType::GamingInput => 12,
            DeviceType::Pen => 13,
            DeviceType::Touchpad => 14,
            DeviceType::Modem => 15,
            DeviceType::Network => 16,
            DeviceType::Headset => 17,
            DeviceType::Speakers => 18,
            DeviceType::Headphones => 19,
            DeviceType::Video => 20,
            DeviceType::OtherAudio => 21,
            DeviceType::RemoteControl => 22,
            DeviceType::Printer => 23,
            DeviceType::Scanner => 24,
            DeviceType::Camera => 25,
            DeviceType::Wearable => 26,
            DeviceType::Toy => 27,
            DeviceType::BluetoothGeneric => 28,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningLevel {
    Unknown,
    None,
    Discharging,
    Low,
    Critical,
    Action,
}

impl From<u32> for WarningLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => WarningLevel::Unknown,
            1 => WarningLevel::None,
            2 => WarningLevel::Discharging,
            3 => WarningLevel::Low,
            4 => WarningLevel::Critical,
            5 => WarningLevel::Action,
            _ => panic!("Invalid warning level"),
        }
    }
}

pub async fn get_battery() -> Result<DeviceProxy<'static>> {
    let connection = Connection::system().await?;
    let upower_p = UPowerProxy::builder(&connection).build().await?;
    let devices = upower_p.enumerate_devices().await?;
    let mut battery = None;
    for device_path in devices {
        let device = DeviceProxy::builder(&connection)
            .path(device_path)?
            .build()
            .await?;
        if device.type_().await? == DeviceType::Battery.into() {
            battery = Some(device);
            break;
        }
    }

    if battery.is_none() {
        return Err(anyhow::Error::msg("Battery not found"));
    }

    Ok(battery.unwrap())
}
