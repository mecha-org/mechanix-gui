use const_format::concatcp;

pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const BASE_SETTINGS_PATH : &str = "/mechanix/home-screen/settings.yml";

pub const ASSET_PATH  :  &str  = concatcp!(USR_SHARE_PATH ,"/mechanix/home-screen/assets/");

pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");
pub const MECHA_CONNECT_ICON : &str = concatcp!(ASSET_PATH, "mecha_connect_icon.png");
pub const MECHA_LLAMA_ICON : &str = concatcp!(ASSET_PATH, "mecha_llama_icon.png");
pub const MECHA_VISION_ICON : &str = concatcp!(ASSET_PATH, "mecha_vision_icon.png");
pub const MECHA_TERMINAL_ICON : &str = concatcp!(ASSET_PATH, "mecha_terminal_icon.png");
pub const MECHA_GAMING_ICON : &str = concatcp!(ASSET_PATH, "mecha_gaming_icon.png");