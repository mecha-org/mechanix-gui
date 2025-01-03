use const_format::concatcp;

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH: &str = ".config";
pub const USR_SHARE_PATH: &str = "/usr/share";
pub const APP_PATH: &str = "/mechanix/shell/keyboard";

pub const BASE_SETTINGS_PATH: &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSETS_PATH: &str = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/");
pub const ICONS_PATH: &str = concatcp!(ASSETS_PATH, "icons/");
pub const LAYOUTS_PATH: &str = concatcp!(ASSETS_PATH, "layouts/");
pub const TERMINAL_PATH: &str = concatcp!(LAYOUTS_PATH, "terminal/");
pub const EMAIL_PATH: &str = concatcp!(LAYOUTS_PATH, "email/");
pub const URL_PATH: &str = concatcp!(LAYOUTS_PATH, "url/");
pub const TRIE_PATH: &str = concatcp!(ASSETS_PATH, "trie/");

pub const LAYOUT_EXAMPLE_PATH: &str = concatcp!(LAYOUTS_PATH, "us.yaml");
pub const TERMINAL_EXAMPLE_PATH: &str = concatcp!(TERMINAL_PATH, "us.yaml");
pub const EMAIL_EXAMPLE_PATH: &str = concatcp!(EMAIL_PATH, "us.yaml");
pub const URL_EXAMPLE_PATH: &str = concatcp!(URL_PATH, "us.yaml");
pub const TRIE_RAW_FILE: &str = concatcp!(TRIE_PATH, "words_raw.tsv");
pub const TRIE_CACHED_FILE: &str =
    concatcp!(HOME_DIR_CONFIG_PATH, APP_PATH, "/caches/words_cached.json");

pub const EDIT_CLEAR_ICON: &str = concatcp!(ICONS_PATH, "edit_clear_icon.svg");
pub const KEY_SHIFT_ICON: &str = concatcp!(ICONS_PATH, "key_shift_icon.svg");
pub const KEY_ENTER_ICON: &str = concatcp!(ICONS_PATH, "key_enter_icon.svg");
pub const KEYBOARD_MODE_ICON: &str = concatcp!(ICONS_PATH, "keyboard_mode_icon.svg");
pub const WINDOW_MAX_ICON: &str = concatcp!(ICONS_PATH, "window_max_icon.svg");
pub const WINDOW_MIN_ICON: &str = concatcp!(ICONS_PATH, "window_min_icon.svg");
