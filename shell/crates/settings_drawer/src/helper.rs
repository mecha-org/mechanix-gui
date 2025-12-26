use crate::ui::icon::IconName;

pub fn get_wireless_strength_icon(
    enabled: bool,
    signal_strength: u8,
    security: String,
) -> IconName {
    let level = if signal_strength == 0 {
        "none"
    } else if signal_strength <= 30 {
        "low"
    } else if signal_strength <= 60 {
        "medium"
    } else if signal_strength <= 100 {
        "high"
    } else {
        "unknown"
    };

    let is_protected = security == "Protected";

    if enabled {
        if is_protected {
            match level {
                "low" => IconName::ConnectedWirelessLowLocked,
                "medium" => IconName::ConnectedWirelessMediumLocked,
                "high" => IconName::ConnectedWirelessHighLocked,
                _ => IconName::ConnectedWirelessWarning,
            }
        } else {
            match level {
                "none" => IconName::ConnectedWirelessOn,
                "low" => IconName::ConnectedWirelessLow,
                "medium" => IconName::ConnectedWirelessMedium,
                "high" => IconName::ConnectedWirelessHigh,
                _ => IconName::ConnectedWirelessWarning,
            }
        }
    } else {
        if is_protected {
            match level {
                "low" => IconName::WirelessLowLocked,
                "medium" => IconName::WirelessMediumLocked,
                "high" => IconName::WirelessHighLocked,
                _ => IconName::WirelessWarning,
            }
        } else {
            match level {
                "none" => IconName::WirelessOn,
                "low" => IconName::WirelessLow,
                "medium" => IconName::WirelessMedium,
                "high" => IconName::WirelessHigh,
                _ => IconName::WirelessWarning,
            }
        }
    }
}

pub fn get_bluetooth_icon(connected: bool) -> IconName {
    if connected {
        IconName::BluetoothConnected
    } else {
        IconName::BluetoothOff
    }
}
