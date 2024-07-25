use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const APP_PATH : &str = "/mechanix/app-drawer";

pub const BASE_SETTINGS_PATH : &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH : &str  = concatcp!(USR_SHARE_PATH, APP_PATH,"/assets/");

pub const SEARCH_ICON : &str = concatcp!(ASSET_PATH, "search_icon.svg");
pub const HOME_ICON : &str = concatcp!(ASSET_PATH, "home_icon.svg");
pub const BACK_ICON : &str = concatcp!(ASSET_PATH, "back_icon.svg");
pub const CLEAR_ICON : &str = concatcp!(ASSET_PATH, "clear_icon.svg");
pub const NOT_FOUND_ICON : &str = concatcp!(ASSET_PATH, "not_found_icon.png");
pub const NOT_FOUND_SMALL_ICON : &str = concatcp!(ASSET_PATH, "not_found_small_icon.png");