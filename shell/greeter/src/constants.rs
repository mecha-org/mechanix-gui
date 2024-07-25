use const_format::concatcp;

pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const BASE_SETTINGS_PATH : &str = "/mechanix/greeter/settings.yml";

pub const ASSET_PATH  :  &str  = concatcp!(USR_SHARE_PATH ,"/mechanix/greeter/assets/");

pub const HOME_ICON : &str = concatcp!(ASSET_PATH, "home_icon.svg");
pub const BACKSPACE_ICON : &str = concatcp!(ASSET_PATH, "backspace_icon.svg");
pub const LOCK_ICON : &str = concatcp!(ASSET_PATH, "lock_icon.svg");
pub const POWER_ICON : &str = concatcp!(ASSET_PATH, "power_icon.svg");
pub const SHUTDOWN_ICON : &str = concatcp!(ASSET_PATH, "shutdown_icon.svg");
pub const RESTART_ICON : &str = concatcp!(ASSET_PATH, "restart_icon.svg");
pub const SLEEP_ICON : &str = concatcp!(ASSET_PATH, "sleep_icon.svg");
pub const CLOSE_ICON : &str = concatcp!(ASSET_PATH, "close_icon.svg");
pub const UNLOCK_ICON : &str = concatcp!(ASSET_PATH, "unlock_icon.svg");
pub const BACK_ICON : &str = concatcp!(ASSET_PATH, "back_icon.svg");
pub const NEXT_ICON : &str = concatcp!(ASSET_PATH, "next_icon.svg");
pub const SUBMIT_ICON : &str = concatcp!(ASSET_PATH, "submit_icon.svg");

pub const PEEK_PASSWORD_ICON : &str = concatcp!(ASSET_PATH, "peek_password_icon.png");
pub const UNPEEK_PASSWORD_ICON : &str = concatcp!(ASSET_PATH, "un_peek_password_icon.png");

pub const SHOW_ICON : &str = concatcp!(ASSET_PATH, "show_icon.svg");
pub const HIDE_ICON : &str = concatcp!(ASSET_PATH, "hide_icon.svg");
pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");

pub const PASSWORD_LENGTH : usize = 4;