/*! The symbol object, defining actions that the key can do when activated */

use bitflags::bitflags;
use std::ffi::CString;

/// Name of the keysym
#[derive(Debug, Clone, PartialEq)]
pub struct KeySym(pub String);

/// Use to switch views
type View = String;

/// Use to send modified keypresses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
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
pub enum Action {
    /// Switch to this view
    SetView(View),
    /// Switch to a view and latch
    LockView {
        lock: View,
        /// When unlocked by pressing it or emitting a key
        unlock: View,
        /// Whether key has a latched state
        /// that pops when another key is pressed.
        latches: bool,
        /// Should take on *locked* appearance whenever latch comes back to those views.
        looks_locked_from: Vec<View>,
    },
    /// Hold this modifier for as long as the button is pressed
    ApplyModifier(Modifier),
    /// Submit some text
    Submit {
        /// Text to submit with input-method.
        /// If None, then keys are to be submitted instead.
        text: Option<String>,
        /// The key events this symbol submits when submitting text is not possible
        keys: Vec<KeySym>,
    },
    /// Erase a position behind the cursor
    Erase,
    ShowPreferences,
    Minimize,
    Maximize,
}

impl Action {
    pub fn is_locked(&self, view_name: &str) -> bool {
        match self {
            Action::LockView {
                lock,
                unlock: _,
                latches: _,
                looks_locked_from: _,
            } => lock == view_name,
            _ => false,
        }
    }
    pub fn has_locked_appearance_from(&self, locked_view_name: &str) -> bool {
        match self {
            Action::LockView {
                lock: _,
                unlock: _,
                latches: _,
                looks_locked_from,
            } => looks_locked_from
                .iter()
                .find(|view| locked_view_name == view.as_str())
                .is_some(),
            _ => false,
        }
    }
    pub fn is_active(&self, view_name: &str) -> bool {
        match self {
            Action::SetView(view) => view == view_name,
            Action::LockView {
                lock,
                unlock: _,
                latches: _,
                looks_locked_from: _,
            } => lock == view_name,
            _ => false,
        }
    }
}
