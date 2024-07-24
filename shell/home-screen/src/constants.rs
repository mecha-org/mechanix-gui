use const_format::concatcp;

pub const ASSET_PATH : &str = "/usr/share/mechanix/home-screen/assets";
pub const HOME_DIR_PATH : &str = ".config/mechanix/home-screen/settings.yml";

pub const BACKGROUND_IMAGE : &str = concatcp!(ASSET_PATH, "mecha_background_color.png");
pub const MECHA_CONNECT_ICON : &str = concatcp!(ASSET_PATH, "mecha_connect_icon.png");
pub const MECHA_LLAMA_ICON : &str = concatcp!(ASSET_PATH, "mecha_llama_icon.png");
pub const MECHA_VISION_ICON : &str = concatcp!(ASSET_PATH, "mecha_vision_icon.png");
pub const MECHA_TERMINAL_ICON : &str = concatcp!(ASSET_PATH, "mecha_terminal_icon.png");
pub const MECHA_GAMING_ICON : &str = concatcp!(ASSET_PATH, "mecha_gaming_icon.png");