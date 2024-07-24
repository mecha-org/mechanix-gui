use const_format::concatcp;

pub const ASSET_PATH  :  &str  = "/usr/share/mechanix/notification/assets/";
pub const HOME_DIR_PATH : &str = ".config/mechanix/notification/settings.yml";

pub const BELL_ICON : &str = concatcp!(ASSET_PATH, "bell_icon.svg");