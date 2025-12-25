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
                "low" => IconName::ConnectedWifiLowLocked,
                "medium" => IconName::ConnectedWifiMediumLocked,
                "high" => IconName::ConnectedWifiHighLocked,
                _ => IconName::ConnectedWifiWarning,
            }
        } else {
            match level {
                "none" => IconName::ConnectedWifiOn,
                "low" => IconName::ConnectedWifiLow,
                "medium" => IconName::ConnectedWifiMedium,
                "high" => IconName::ConnectedWifiHigh,
                _ => IconName::ConnectedWifiWarning,
            }
        }
    } else {
        if is_protected {
            match level {
                "low" => IconName::WifiLowLocked,
                "medium" => IconName::WifiMediumLocked,
                "high" => IconName::WifiHighLocked,
                _ => IconName::WifiWarning,
            }
        } else {
            match level {
                "none" => IconName::WifiOn,
                "low" => IconName::WifiLow,
                "medium" => IconName::WifiMedium,
                "high" => IconName::WifiHigh,
                _ => IconName::WifiWarning,
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
