mod events;
pub mod services;
mod ui;

pub mod prelude {
    pub use crate::events::{AppEvents, BrightnessEvents, BtEvents, NmEvents, VolumeEvents};
    pub use crate::ui::SettingsDrawer;
}

use crate::ui::icon::IconName;


pub fn get_wireless_strength_icon(signal_strength: u8) -> IconName {
    match signal_strength {
        0 => IconName::WirelessOn,
        1..=30 => IconName::WirelessLow,
        31..=60 => IconName::WirelessMedium,
        61..=85 => IconName::WirelessHigh,
        86..=100 => IconName::WirelessFull,
        _ => IconName::WirelessWarning,
    }
}
