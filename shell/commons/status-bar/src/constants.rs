use const_format::concatcp;
// status-bar 
const ASSET_PATH  :  &str  = "/usr/share/mechanix/status-bar/assets/";

// battery
pub const BATTERY_LEVEL_100 : &str = concatcp!(ASSET_PATH, "battery/battery_100_icon.svg");
pub const BATTERY_LEVEL_90 : &str = concatcp!(ASSET_PATH, "battery/battery_90_icon.svg");
pub const BATTERY_LEVEL_80 : &str = concatcp!(ASSET_PATH, "battery/battery_80_icon.svg");
pub const BATTERY_LEVEL_70 : &str = concatcp!(ASSET_PATH, "battery/battery_70_icon.svg");
pub const BATTERY_LEVEL_60 : &str = concatcp!(ASSET_PATH, "battery/battery_60_icon.svg");
pub const BATTERY_LEVEL_50 : &str = concatcp!(ASSET_PATH, "battery/battery_50_icon.svg");
pub const BATTERY_LEVEL_40 : &str = concatcp!(ASSET_PATH, "battery/battery_40_icon.svg");
pub const BATTERY_LEVEL_30 : &str = concatcp!(ASSET_PATH, "battery/battery_30_icon.svg");
pub const BATTERY_LEVEL_20 : &str = concatcp!(ASSET_PATH, "battery/battery_20_icon.svg");
pub const BATTERY_LEVEL_10 : &str = concatcp!(ASSET_PATH, "battery/battery_10_icon.svg");
pub const BATTERY_LEVEL_0 : &str = concatcp!(ASSET_PATH, "battery/battery_0_icon.svg");
pub const BATTERY_NOT_FOUND : &str = concatcp!(ASSET_PATH, "battery/battery_not_found_icon.svg");

// charging-battery
pub const CHARGING_BATTERY_LEVEL_100 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_100_icon.svg");
pub const CHARGING_BATTERY_LEVEL_90 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_90_icon.svg");
pub const CHARGING_BATTERY_LEVEL_80 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_80_icon.svg");
pub const CHARGING_BATTERY_LEVEL_70 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_70_icon.svg");
pub const CHARGING_BATTERY_LEVEL_60 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_60_icon.svg");
pub const CHARGING_BATTERY_LEVEL_50 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_50_icon.svg");
pub const CHARGING_BATTERY_LEVEL_40 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_40_icon.svg");
pub const CHARGING_BATTERY_LEVEL_30 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_30_icon.svg");
pub const CHARGING_BATTERY_LEVEL_20 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_20_icon.svg");
pub const CHARGING_BATTERY_LEVEL_10 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_10_icon.svg");
pub const CHARGING_BATTERY_LEVEL_0 : &str = concatcp!(ASSET_PATH, "battery/charging/battery_0_icon.svg");

// wireless
pub const WIRELESS_OFF : &str = concatcp!(ASSET_PATH, "wireless/wireless_off_icon.svg");
pub const WIRELESS_ON : &str = concatcp!(ASSET_PATH, "wireless/wireless_on_icon.svg");
pub const WIRELESS_LOW : &str = concatcp!(ASSET_PATH, "wireless/wireless_low_icon.svg");
pub const WIRELESS_WEAK : &str = concatcp!(ASSET_PATH, "wireless/wireless_weak_icon.svg");
pub const WIRELESS_GOOD : &str = concatcp!(ASSET_PATH, "wireless/wireless_good_icon.svg");
pub const WIRELESS_STRONG : &str = concatcp!(ASSET_PATH, "wireless/wireless_strong_icon.svg");
pub const WIRELESS_NOT_FOUND : &str = concatcp!(ASSET_PATH, "wireless/wireless_not_found_icon.svg");

// bluetooth
pub const BLUETOOTH_ON : &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_on_icon.svg");
pub const BLUETOOTH_OFF : &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_off_icon.svg");
pub const BLUETOOTH_CONNECTED : &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_connected_icon.svg");
pub const BLUETOOTH_NOT_FOUND : &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_not_found_icon.svg");