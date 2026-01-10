use anyhow::Result;
use bitflags::bitflags;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::string::FromUtf8Error;
use std::{fmt, io};
use xkbcommon::xkb;

/// Name of the keysym
#[derive(Debug, Clone, PartialEq)]
pub struct KeySym(pub String);

/// Use to switch views
pub type ViewString = String;

/// Use to send modified keypresses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierParsed {
    /// Control and Alt are the only modifiers
    /// which doesn't interfere with levels,
    Control,
    Alt,
    Mod4,
}

bitflags! {
    /// Map to `virtual_keyboard.modifiers` modifiers values
    /// From https://www.x.org/releases/current/doc/kbproto/xkbproto.html#Keyboard_State
    pub struct Modifiers: u8 {
        const SHIFT = 0x1;
        const LOCK = 0x2;
        const CONTROL = 0x4;
        /// Alt
        const MOD1 = 0x8;
        const MOD2 = 0x10;
        const MOD3 = 0x20;
        /// Meta
        const MOD4 = 0x40;
        /// AltGr
        const MOD5 = 0x80;
    }
}

/// Action to perform on the keypress and, in reverse, on keyrelease
#[derive(Debug, Clone, PartialEq)]
pub enum ActionParsed {
    /// Switch to this view
    SetView(ViewString),
    /// Switch to a view and latch
    LockView {
        lock: ViewString,
        /// When unlocked by pressing it or emitting a key
        unlock: ViewString,
        /// Whether key has a latched state
        /// that pops when another key is pressed.
        latches: bool,
        /// Should take on *locked* appearance whenever latch comes back to those views.
        looks_locked_from: Vec<ViewString>,
    },
    /// Hold this modifier for as long as the button is pressed
    ApplyModifier(ModifierParsed),
    /// Submit some text
    Submit {
        /// Text to submit with input-method.
        /// If None, then keys are to be submitted instead.
        text: Option<String>,
        /// The key events this symbol submits when submitting text is not possible
        keysym: Option<String>,
    },
    /// Erase a position behind the cursor
    Erase,
    ShowPreferences,
    Minimize,
    Maximize,
}

impl ActionParsed {
    pub fn is_locked(&self, view_name: &str) -> bool {
        match self {
            ActionParsed::LockView { lock, .. } => lock == view_name,
            _ => false,
        }
    }

    pub fn has_locked_appearance_from(&self, locked_view_name: &str) -> bool {
        match self {
            ActionParsed::LockView {
                looks_locked_from, ..
            } => looks_locked_from
                .iter()
                .any(|view| locked_view_name == view.as_str()),
            _ => false,
        }
    }

    pub fn is_active(&self, view_name: &str) -> bool {
        match self {
            ActionParsed::SetView(view) => view == view_name,
            ActionParsed::LockView { lock, .. } => lock == view_name,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Label {
    Text(String),
    Icon(String),
}

/// The extended, unambiguous layout-keycode
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct KeyCode {
    pub code: u32,
    pub keymap_idx: usize,
}

#[derive(Debug, Clone)]
pub struct Key {
    /// A cache of raw keycodes derived from Action::Submit given a keymap
    pub keycodes: Vec<KeyCode>,
    /// Static description of what the key does when pressed or released
    pub action: ActionParsed,
}

#[derive(Debug)]
pub enum FormattingError {
    Utf(FromUtf8Error),
    Format(io::Error),
}

impl fmt::Display for FormattingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattingError::Utf(e) => write!(f, "UTF: {}", e),
            FormattingError::Format(e) => write!(f, "Format: {}", e),
        }
    }
}

impl From<io::Error> for FormattingError {
    fn from(e: io::Error) -> Self {
        FormattingError::Format(e)
    }
}

impl std::error::Error for FormattingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FormattingError::Utf(e) => Some(e),
            FormattingError::Format(e) => Some(e),
        }
    }
}
