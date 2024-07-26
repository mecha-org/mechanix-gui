use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const APP_PATH : &str = "/mechanix/shell/settings-panel";

pub const BASE_SETTINGS_PATH : &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH : &str  = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/");

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


// rotation 
pub const ROTATION_PORTRAIT : &str = concatcp!(ASSET_PATH, "rotation/portrait_icon.svg"); 
pub const ROTATION_LANDSCAPE : &str = concatcp!(ASSET_PATH, "rotation/landscape_icon.svg"); 

// settings
pub const SETTINGS_ICON : &str = concatcp!(ASSET_PATH, "settings/settings_icon.svg");

// running-apps
pub const RUNNING_APPS_LOW : &str = concatcp!(ASSET_PATH, "running_apps/low_icon.svg");
pub const RUNNING_APPS_MEDIUM : &str = concatcp!(ASSET_PATH, "running_apps/medium_icon.svg");
pub const RUNNING_APPS_HIGH : &str = concatcp!(ASSET_PATH, "running_apps/high_icon.svg");

// cpu
pub const CPU_LOW : &str = concatcp!(ASSET_PATH, "cpu/low_icon.svg");
pub const CPU_MEDIUM : &str = concatcp!(ASSET_PATH, "cpu/medium_icon.svg");
pub const CPU_HIGH : &str = concatcp!(ASSET_PATH, "cpu/high_icon.svg");

// memory
pub const MEMORY_LOW : &str = concatcp!(ASSET_PATH, "memory/low_icon.svg");
pub const MEMORY_MEDIUM : &str = concatcp!(ASSET_PATH, "memory/medium_icon.svg");
pub const MEMORY_HIGH : &str = concatcp!(ASSET_PATH, "memory/high_icon.svg");

// sound
pub const SOUND_LOW : &str = concatcp!(ASSET_PATH, "sound/low_icon.svg");
pub const SOUND_MEDIUM : &str = concatcp!(ASSET_PATH, "sound/medium_icon.svg");
pub const SOUND_HIGH : &str = concatcp!(ASSET_PATH, "sound/high_icon.svg");

// brightness
pub const BRIGHTNESS_LOW : &str = concatcp!(ASSET_PATH, "brightness/low_icon.svg");
pub const BRIGHTNESS_MEDIUM : &str = concatcp!(ASSET_PATH, "brightness/medium_icon.svg");
pub const BRIGHTNESS_HIGH : &str = concatcp!(ASSET_PATH, "brightness/high_icon.svg");
