use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const APP_PATH : &str = "/mechanix/power-options";

pub const BASE_SETTINGS_PATH : &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH : &str  = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/");

pub const SHUTDOWN_ICON : &str = concatcp!(ASSET_PATH, "shutdown_icon.svg");
pub const RESTART_ICON : &str = concatcp!(ASSET_PATH, "restart_icon.svg");
pub const LOGOUT_ICON : &str = concatcp!(ASSET_PATH, "logout_icon.svg");
pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");