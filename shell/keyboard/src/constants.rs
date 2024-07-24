use const_format::concatcp;

pub const ASSET_PATH : &str  = "/usr/share/mechanix/keyboard/assets/";
pub const LAYOUT_EXAMPLE_PATH : &str = concatcp!(ASSET_PATH, "layouts/us.yml");

pub const TRIE_RAW_FILE : &str = concatcp!(ASSET_PATH, "trie/words_raw.tsv");
pub const TRIE_CACHED_FILE : &str = concatcp!(ASSET_PATH, "trie/words_cached.json");

pub const HOME_DIR_PATH : &str = ".config/mechanix/keyboard/settings.yml";

pub const EDIT_CLEAR_ICON : &str = concatcp!(ASSET_PATH, "edit_clear_icon.svg");
pub const KEY_SHIFT_ICON : &str = concatcp!(ASSET_PATH, "key_shift_icon.svg");
pub const KEY_ENTER_ICON : &str = concatcp!(ASSET_PATH, "key_enter_icon.svg");
pub const KEYBOARD_MODE_ICON : &str = concatcp!(ASSET_PATH, "keyboard_mode_icon.svg");