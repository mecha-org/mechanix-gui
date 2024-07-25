use const_format::concatcp;

pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const BASE_SETTINGS_PATH : &str = "/mechanix/lock-screen/settings.yml";

pub const ASSET_PATH  :  &str  = concatcp!(USR_SHARE_PATH ,"/mechanix/lock-screen/assets/");

pub const HOME_ICON : &str = concatcp!(ASSET_PATH, "home_icon.svg");
pub const BACKSPACE_ICON : &str = concatcp!(ASSET_PATH, "backspace_icon.svg");
pub const BACK_ICON : &str = concatcp!(ASSET_PATH, "back_icon.svg");
pub const LOCK_ICON : &str = concatcp!(ASSET_PATH, "lock_icon.svg");
pub const UNLOCK_ICON : &str = concatcp!(ASSET_PATH, "unlock_icon.svg");
pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");

pub const PASSWORD_LENGTH : usize = 4;