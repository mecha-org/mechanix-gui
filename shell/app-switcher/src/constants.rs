use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const APP_PATH : &str = "/mechanix/app-switcher";

pub const BASE_SETTINGS_PATH : &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH : &str  = concatcp!(USR_SHARE_PATH , APP_PATH, "/assets/");

pub const APP_NAMESPACE : &str = concatcp!(ASSET_PATH, "mechanix.shell.home-screen");
pub const BECK_ICON : &str = concatcp!(ASSET_PATH, "back_icon.svg");
pub const CLOSE_ALL_ICON : &str = concatcp!(ASSET_PATH, "close_all_icon.svg");
pub const NOT_FOUND_ICON : &str = concatcp!(ASSET_PATH, "not_found_small_icon.png");