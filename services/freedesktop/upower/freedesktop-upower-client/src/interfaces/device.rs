/// Represents the battery level of a device.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BatteryLevel {
    /// The battery level is unknown.
    #[default]
    Unknown,
    /// The battery level is not applicable (e.g., the device does not use coarse level reporting).
    None,
    /// The battery level is low.
    Low,
    /// The battery level is critical.
    Critical,
    /// The battery level is normal.
    Normal,
    /// The battery level is high.
    High,
    /// The battery level is full.
    Full,
}
impl From<u32> for BatteryLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => BatteryLevel::Unknown,
            1 => BatteryLevel::None,
            3 => BatteryLevel::Low,
            4 => BatteryLevel::Critical,
            6 => BatteryLevel::Normal,
            7 => BatteryLevel::High,
            8 => BatteryLevel::Full,
            _ => BatteryLevel::Unknown,
        }
    }
}

/// Represents the various possible states of a battery.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BatteryState {
    /// The battery status is unknown.
    #[default]
    Unknown,
    /// The battery is currently charging.
    Charging,
    /// The battery is discharging (being used).
    Discharging,
    /// The battery is empty.
    Empty,
    /// The battery is fully charged.
    FullCharged,
    /// The battery is pending charge.
    PendingCharge,
    /// The battery is pending discharge.
    PendingDischarge,
}

impl From<u32> for BatteryState {
    fn from(value: u32) -> Self {
        match value {
            0 => BatteryState::Unknown,
            1 => BatteryState::Charging,
            2 => BatteryState::Discharging,
            3 => BatteryState::Empty,
            4 => BatteryState::FullCharged,
            5 => BatteryState::PendingCharge,
            6 => BatteryState::PendingDischarge,
            _ => BatteryState::Unknown
        }
    }
}

/// Represents the warning level associated with the battery or power source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningLevel {
    /// The warning level is unknown.
    Unknown,
    /// No warning; the battery or power source is in a normal state.
    None,
    /// The battery is discharging, but not yet at a low level.
    Discharging,
    /// The battery level is low and may require attention soon.
    Low,
    /// The battery level is critical and requires immediate attention.
    Critical,
    /// Immediate action is required due to extremely low battery level.
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
/// Represents the type of power source as defined by UPower.
///
/// Each variant corresponds to a specific hardware or logical power source,
/// mapped directly to its associated integer value in the UPower specification.
///
/// Use [`TryFrom<u32>`](std::convert::TryFrom) to safely convert from a raw integer
/// (such as received via D-Bus) to a `PowerSourceType`. Returns `Err(())` if the
/// value does not correspond to a known variant.
///
/// # Variants
/// - `Unknown` (0): Unknown power source type.
/// - `LinePower` (1): Line power (AC).
/// - `Battery` (2): Battery.
/// - `Ups` (3): Uninterruptible Power Supply.
/// - `Monitor` (4): Monitor.
/// - `Mouse` (5): Mouse.
/// - `Keyboard` (6): Keyboard.
/// - `Pda` (7): Personal Digital Assistant.
/// - `Phone` (8): Phone.
/// - `MediaPlayer` (9): Media player.
/// - `Tablet` (10): Tablet.
/// - `Computer` (11): Computer.
/// - `GamingInput` (12): Gaming input device.
/// - `Pen` (13): Pen.
/// - `Touchpad` (14): Touchpad.
/// - `Modem` (15): Modem.
/// - `Network` (16): Network device.
/// - `Headset` (17): Headset.
/// - `Speakers` (18): Speakers.
/// - `Headphones` (19): Headphones.
/// - `Video` (20): Video device.
/// - `OtherAudio` (21): Other audio device.
/// - `RemoteControl` (22): Remote control.
/// - `Printer` (23): Printer.
/// - `Scanner` (24): Scanner.
/// - `Camera` (25): Camera.
/// - `Wearable` (26): Wearable device.
/// - `Toy` (27): Toy.
/// - `BluetoothGeneric` (28): Generic Bluetooth device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSourceType {
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

/// Attempts to convert a `u32` value into a [`PowerSourceType`].
///
/// Returns `Ok(PowerSourceType)` if the value matches a known variant,
/// or `Err(())` if the value is not recognized.
///
/// # Example
/// ```ignore
/// use std::convert::TryFrom;
/// let t = PowerSourceType::try_from(2);
/// assert_eq!(t, Ok(PowerSourceType::Battery));
/// ```
impl std::convert::TryFrom<u32> for PowerSourceType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerSourceType::Unknown),
            1 => Ok(PowerSourceType::LinePower),
            2 => Ok(PowerSourceType::Battery),
            3 => Ok(PowerSourceType::Ups),
            4 => Ok(PowerSourceType::Monitor),
            5 => Ok(PowerSourceType::Mouse),
            6 => Ok(PowerSourceType::Keyboard),
            7 => Ok(PowerSourceType::Pda),
            8 => Ok(PowerSourceType::Phone),
            9 => Ok(PowerSourceType::MediaPlayer),
            10 => Ok(PowerSourceType::Tablet),
            11 => Ok(PowerSourceType::Computer),
            12 => Ok(PowerSourceType::GamingInput),
            13 => Ok(PowerSourceType::Pen),
            14 => Ok(PowerSourceType::Touchpad),
            15 => Ok(PowerSourceType::Modem),
            16 => Ok(PowerSourceType::Network),
            17 => Ok(PowerSourceType::Headset),
            18 => Ok(PowerSourceType::Speakers),
            19 => Ok(PowerSourceType::Headphones),
            20 => Ok(PowerSourceType::Video),
            21 => Ok(PowerSourceType::OtherAudio),
            22 => Ok(PowerSourceType::RemoteControl),
            23 => Ok(PowerSourceType::Printer),
            24 => Ok(PowerSourceType::Scanner),
            25 => Ok(PowerSourceType::Camera),
            26 => Ok(PowerSourceType::Wearable),
            27 => Ok(PowerSourceType::Toy),
            28 => Ok(PowerSourceType::BluetoothGeneric),
            _ => Err(()),
        }
    }
}

// pub async fn get_battery() -> Result<DeviceProxy<'static>> {
//     let connection = Connection::system().await?;
//     let upower_p = UPowerProxy::builder(&connection).build().await?;
//     let devices = upower_p.enumerate_devices().await?;
//     let mut battery = None;
//     for device_path in devices {
//         let device = DeviceProxy::builder(&connection)
//             .path(device_path)?
//             .build()
//             .await?;
//         if device.type_().await? == PowerSourceType::Battery.into() {
//             battery = Some(device);
//             break;
//         }
//     }

//     if battery.is_none() {
//         return Err(anyhow::Error::msg("Battery not found"));
//     }

//     Ok(battery.unwrap())
// }
