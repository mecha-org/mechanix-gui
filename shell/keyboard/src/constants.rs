use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH : &str = "/.config";
pub const USR_SHARE_PATH : &str = "/usr/share";
pub const APP_PATH : &str = "/mechanix/keyboard";

pub const BASE_SETTINGS_PATH : &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH : &str  = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/");

pub const LAYOUT_EXAMPLE_PATH : &str = concatcp!(ASSET_PATH, "layouts/us.yml");
pub const TRIE_RAW_FILE : &str = concatcp!(ASSET_PATH, "trie/words_raw.tsv");
pub const TRIE_CACHED_FILE : &str = concatcp!(ASSET_PATH, "trie/words_cached.json"); 

pub const EDIT_CLEAR_ICON : &str = concatcp!(ASSET_PATH, "edit_clear_icon.svg");
pub const KEY_SHIFT_ICON : &str = concatcp!(ASSET_PATH, "key_shift_icon.svg");
pub const KEY_ENTER_ICON : &str = concatcp!(ASSET_PATH, "key_enter_icon.svg");
pub const KEYBOARD_MODE_ICON : &str = concatcp!(ASSET_PATH, "keyboard_mode_icon.svg");