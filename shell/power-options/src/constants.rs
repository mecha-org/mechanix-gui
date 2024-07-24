use const_format::concatcp;

pub const ASSET_PATH  :  &str  = "/usr/share/mechanix/power-options/icons/";
pub const HOME_DIR_PATH : &str = ".config/mechanix/power-options/settings.yml";

pub const SHUTDOWN_ICON : &str = concatcp!(ASSET_PATH, "shutdown_icon.svg");
pub const RESTART_ICON : &str = concatcp!(ASSET_PATH, "restart_icon.svg");
pub const LOGOUT_ICON : &str = concatcp!(ASSET_PATH, "logout_icon.svg");
pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");