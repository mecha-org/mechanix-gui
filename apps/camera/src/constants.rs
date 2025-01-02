use const_format::concatcp;

pub const APP_ID: &str = "mechanix-camera";

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH: &str = "/.config";
pub const USR_SHARE_PATH: &str = "/usr/share";
pub const APP_PATH: &str = "/mechanix/apps/settings-app";

pub const BASE_SETTINGS_PATH: &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH: &str = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/"); // OG
pub const BACKGROUND_IMAGE: &str = concatcp!(ASSET_PATH, "mecha_background_color.png");

pub const BACK_ICON: &str = concatcp!(ASSET_PATH, "back_icon.png");
