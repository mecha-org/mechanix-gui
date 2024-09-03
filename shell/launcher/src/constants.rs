use const_format::concatcp;

pub const APP_ID: &str = "mechanix.shell.launcher";

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
pub const SEARCH_ICON: &str = concatcp!(ASSET_PATH, "searchicon.svg");
pub const SETTINGS_ICON: &str = concatcp!(ASSET_PATH, "settings_icon.svg");
pub const LAUNCH_ICON: &str = concatcp!(ASSET_PATH, "launch_icon.svg");
pub const DELETE_ICON: &str = concatcp!(ASSET_PATH, "delete_icon.svg");
pub const CLOSE_ICON: &str = concatcp!(ASSET_PATH, "close_icon.svg");
pub const TERMINAL_ICON: &str = concatcp!(ASSET_PATH, "terminal_icon.svg");

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

pub const WIRELESS: &str = "wireless";
pub const SM: &str = "/sm/";
pub const LG: &str = "/lg/";

// wireless
pub const SM_WIRELESS_OFF: &str = concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_off_icon.svg");
pub const SM_WIRELESS_ON: &str = concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_on_icon.svg");
pub const SM_WIRELESS_LOW: &str = concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_low_icon.svg");
pub const SM_WIRELESS_WEAK: &str = concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_weak_icon.svg");
pub const SM_WIRELESS_GOOD: &str = concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_good_icon.svg");
pub const SM_WIRELESS_STRONG: &str =
    concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_strong_icon.svg");
pub const SM_WIRELESS_NOT_FOUND: &str =
    concatcp!(ASSET_PATH, WIRELESS, SM, "wireless_not_found_icon.svg");

pub const LG_WIRELESS_OFF: &str = concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_off_icon.svg");
pub const LG_WIRELESS_ON: &str = concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_on_icon.svg");
pub const LG_WIRELESS_LOW: &str = concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_low_icon.svg");
pub const LG_WIRELESS_WEAK: &str = concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_weak_icon.svg");
pub const LG_WIRELESS_GOOD: &str = concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_good_icon.svg");
pub const LG_WIRELESS_STRONG: &str =
    concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_strong_icon.svg");
pub const LG_WIRELESS_NOT_FOUND: &str =
    concatcp!(ASSET_PATH, WIRELESS, LG, "wireless_not_found_icon.svg");

// bluetooth
//sm
pub const BLUETOOTH: &str = "bluetooth";
pub const SM_BLUETOOTH_ON: &str = concatcp!(ASSET_PATH, BLUETOOTH, SM, "bluetooth_on_icon.svg");
pub const SM_BLUETOOTH_OFF: &str = concatcp!(ASSET_PATH, BLUETOOTH, SM, "bluetooth_off_icon.svg");
pub const SM_BLUETOOTH_CONNECTED: &str =
    concatcp!(ASSET_PATH, BLUETOOTH, SM, "bluetooth_connected_icon.svg");
pub const SM_BLUETOOTH_NOT_FOUND: &str =
    concatcp!(ASSET_PATH, BLUETOOTH, SM, "bluetooth_not_found_icon.svg");

//lg
pub const LG_BLUETOOTH_ON: &str = concatcp!(ASSET_PATH, BLUETOOTH, LG, "bluetooth_on_icon.svg");
pub const LG_BLUETOOTH_OFF: &str = concatcp!(ASSET_PATH, BLUETOOTH, LG, "bluetooth_off_icon.svg");
pub const LG_BLUETOOTH_CONNECTED: &str =
    concatcp!(ASSET_PATH, BLUETOOTH, LG, "bluetooth_connected_icon.svg");
pub const LG_BLUETOOTH_NOT_FOUND: &str =
    concatcp!(ASSET_PATH, BLUETOOTH, LG, "bluetooth_not_found_icon.svg");

//power options
pub const SHUTDOWN_ICON: &str = concatcp!(ASSET_PATH, "shutdown_icon.png");
pub const RESTART_ICON: &str = concatcp!(ASSET_PATH, "restart_icon.png");
