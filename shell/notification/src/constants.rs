use const_format::concatcp;

pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const BASE_SETTINGS_PATH : &str = "/mechanix/notification/settings.yml";

pub const ASSET_PATH  :  &str  = concatcp!(USR_SHARE_PATH ,"/mechanix/notification/assets/");
pub const BELL_ICON : &str = concatcp!(ASSET_PATH, "bell_icon.svg");