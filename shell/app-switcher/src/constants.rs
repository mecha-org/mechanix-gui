use const_format::concatcp;

pub const ASSET_PATH : &str = "/usr/share/mechanix/app-switcher/assets/";
pub const HOME_DIR_PATH : &str = ".config/mechanix/app-switcher/settings.yml";

pub const APP_NAMESPACE : &str = concatcp!(ASSET_PATH, "mechanix.shell.home-screen");
pub const BECK_ICON : &str = concatcp!(ASSET_PATH, "back_icon.svg");
pub const CLOSE_ALL_ICON : &str = concatcp!(ASSET_PATH, "close_all_icon.svg");
pub const NOT_FOUND_ICON : &str = concatcp!(ASSET_PATH, "not_found_small_icon.png");