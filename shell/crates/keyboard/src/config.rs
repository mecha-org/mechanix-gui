use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GapDirection {
    pub default: f64,
    #[serde(default)]
    pub custom: HashMap<String, f64>,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub col: GapDirection,
    #[serde(default)]
    pub row: GapDirection,
}

/// The root element describing an entire keyboard
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    pub views: std::collections::HashMap<String, Vec<ButtonIds>>,
    #[serde(default)]
    pub buttons: std::collections::HashMap<String, ButtonMeta>,
    pub outlines: std::collections::HashMap<String, Outline>,
    #[serde(default)]
    pub gap: Gap,
}

/// Buttons are embedded in a single string
pub type ButtonIds = String;

/// All info about a single button
#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct ButtonMeta {
    /// Special action to perform on activation.
    #[serde(with = "serde_yaml::with::singleton_map", default)]
    pub action: Option<Action>,
    /// The name of the XKB keysym to emit on activation.
    pub keysym: Option<String>,
    /// The text to submit on activation.
    pub text: Option<String>,
    /// The modifier to apply while the key is locked
    pub modifier: Option<Modifier>,
    /// If not present, will be derived from text or the button ID
    pub label: Option<String>,
    /// Conflicts with label
    pub icon: Option<String>,
    /// The name of the outline. If not present, will be "default"
    pub outline: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub enum Action {
    #[serde(rename = "locking")]
    Locking {
        lock_view: String,
        unlock_view: String,
        pops: Option<bool>,
        #[serde(default)]
        looks_locked_from: Vec<String>,
    },
    #[serde(rename = "set_view")]
    SetView(String),
    #[serde(rename = "show_prefs")]
    ShowPrefs,
    #[serde(rename = "erase")]
    Erase,
    #[serde(rename = "minimize")]
    Minimize,
    #[serde(rename = "maximize")]
    Maximize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Modifier {
    Control,
    Shift,
    Lock,
    #[serde(alias = "Mod1")]
    Alt,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Outline {
    pub width: f64,
    pub height: f64,
}
