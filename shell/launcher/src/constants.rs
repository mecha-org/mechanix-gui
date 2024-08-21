use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH: &str = "/.config";
pub const USR_SHARE_PATH: &str = "/usr/share";
pub const APP_PATH: &str = "/mechanix/shell/home-screen";

pub const BASE_SETTINGS_PATH: &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH: &str = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/");

pub const BACKGROUND_IMAGE: &str = concatcp!(ASSET_PATH, "mecha_background_color.png");
pub const MECHA_CONNECT_ICON: &str = concatcp!(ASSET_PATH, "mecha_connect_icon.png");
pub const MECHA_LLAMA_ICON: &str = concatcp!(ASSET_PATH, "mecha_llama_icon.png");
pub const MECHA_VISION_ICON: &str = concatcp!(ASSET_PATH, "mecha_vision_icon.png");
pub const MECHA_TERMINAL_ICON: &str = concatcp!(ASSET_PATH, "mecha_terminal_icon.png");
pub const MECHA_GAMING_ICON: &str = concatcp!(ASSET_PATH, "mecha_gaming_icon.png");

// rotation
pub const ROTATION_PORTRAIT: &str = concatcp!(ASSET_PATH, "rotation/portrait_icon.svg");
pub const ROTATION_LANDSCAPE: &str = concatcp!(ASSET_PATH, "rotation/landscape_icon.svg");
pub const POWER_ICON: &str = concatcp!(ASSET_PATH, "power_icon.svg");
pub const LOCK_ICON: &str = concatcp!(ASSET_PATH, "lock_icon.svg");
pub const SETTINGS_ICON: &str = concatcp!(ASSET_PATH, "settings_icon.svg");

// battery
pub const BATTERY_LEVEL_100: &str = concatcp!(ASSET_PATH, "battery/battery_100_icon.svg");
pub const BATTERY_LEVEL_90: &str = concatcp!(ASSET_PATH, "battery/battery_90_icon.svg");
pub const BATTERY_LEVEL_80: &str = concatcp!(ASSET_PATH, "battery/battery_80_icon.svg");
pub const BATTERY_LEVEL_70: &str = concatcp!(ASSET_PATH, "battery/battery_70_icon.svg");
pub const BATTERY_LEVEL_60: &str = concatcp!(ASSET_PATH, "battery/battery_60_icon.svg");
pub const BATTERY_LEVEL_50: &str = concatcp!(ASSET_PATH, "battery/battery_50_icon.svg");
pub const BATTERY_LEVEL_40: &str = concatcp!(ASSET_PATH, "battery/battery_40_icon.svg");
pub const BATTERY_LEVEL_30: &str = concatcp!(ASSET_PATH, "battery/battery_30_icon.svg");
pub const BATTERY_LEVEL_20: &str = concatcp!(ASSET_PATH, "battery/battery_20_icon.svg");
pub const BATTERY_LEVEL_10: &str = concatcp!(ASSET_PATH, "battery/battery_10_icon.svg");
pub const BATTERY_LEVEL_0: &str = concatcp!(ASSET_PATH, "battery/battery_0_icon.svg");
pub const BATTERY_NOT_FOUND: &str = concatcp!(ASSET_PATH, "battery/battery_not_found_icon.svg");

// charging-battery
pub const CHARGING_BATTERY_LEVEL_100: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_100_icon.svg");
pub const CHARGING_BATTERY_LEVEL_90: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_90_icon.svg");
pub const CHARGING_BATTERY_LEVEL_80: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_80_icon.svg");
pub const CHARGING_BATTERY_LEVEL_70: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_70_icon.svg");
pub const CHARGING_BATTERY_LEVEL_60: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_60_icon.svg");
pub const CHARGING_BATTERY_LEVEL_50: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_50_icon.svg");
pub const CHARGING_BATTERY_LEVEL_40: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_40_icon.svg");
pub const CHARGING_BATTERY_LEVEL_30: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_30_icon.svg");
pub const CHARGING_BATTERY_LEVEL_20: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_20_icon.svg");
pub const CHARGING_BATTERY_LEVEL_10: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_10_icon.svg");
pub const CHARGING_BATTERY_LEVEL_0: &str =
    concatcp!(ASSET_PATH, "battery/charging/battery_0_icon.svg");

// wireless
pub const WIRELESS_OFF: &str = concatcp!(ASSET_PATH, "wireless/wireless_off_icon.svg");
pub const WIRELESS_ON: &str = concatcp!(ASSET_PATH, "wireless/wireless_on_icon.svg");
pub const WIRELESS_LOW: &str = concatcp!(ASSET_PATH, "wireless/wireless_low_icon.svg");
pub const WIRELESS_WEAK: &str = concatcp!(ASSET_PATH, "wireless/wireless_weak_icon.svg");
pub const WIRELESS_GOOD: &str = concatcp!(ASSET_PATH, "wireless/wireless_good_icon.svg");
pub const WIRELESS_STRONG: &str = concatcp!(ASSET_PATH, "wireless/wireless_strong_icon.svg");
pub const WIRELESS_NOT_FOUND: &str = concatcp!(ASSET_PATH, "wireless/wireless_not_found_icon.svg");

// bluetooth
pub const BLUETOOTH_ON: &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_on_icon.svg");
pub const BLUETOOTH_OFF: &str = concatcp!(ASSET_PATH, "bluetooth/bluetooth_off_icon.svg");
pub const BLUETOOTH_CONNECTED: &str =
    concatcp!(ASSET_PATH, "bluetooth/bluetooth_connected_icon.svg");
pub const BLUETOOTH_NOT_FOUND: &str =
    concatcp!(ASSET_PATH, "bluetooth/bluetooth_not_found_icon.svg");
