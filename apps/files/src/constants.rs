use const_format::concatcp;

pub const APP_ID: &str = "mechanix-files";

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH: &str = "/.config";
pub const USR_SHARE_PATH: &str = "/usr/share";
pub const APP_PATH: &str = "/mechanix/apps/files";

pub const BASE_SETTINGS_PATH: &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH: &str = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/"); // OG
pub const BACKGROUND_IMAGE: &str = concatcp!(ASSET_PATH, "mecha_background_color.png");

// Default icon paths
pub const FOLD_ICON: &str = concatcp!(ASSET_PATH, "icons/fold.png");
pub const FILE_ICON: &str = concatcp!(ASSET_PATH, "icons/file.png");
pub const ARROW_ICON: &str = concatcp!(ASSET_PATH, "icons/arrow.png");
pub const BACK_ICON: &str = concatcp!(ASSET_PATH, "icons/back.png");
pub const ADD_ICON: &str = concatcp!(ASSET_PATH, "icons/add.png");
pub const DOTS_ICON: &str = concatcp!(ASSET_PATH, "icons/dots.png");
pub const PDF_ICON: &str = concatcp!(ASSET_PATH, "icons/pdf.png");
pub const IMG_ICON: &str = concatcp!(ASSET_PATH, "icons/image.png");
pub const UNFOLD_ICON: &str = concatcp!(ASSET_PATH, "icons/unfold.png");
