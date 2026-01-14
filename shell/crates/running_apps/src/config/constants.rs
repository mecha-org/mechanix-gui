use std::sync::LazyLock;

use gpui::*;

pub static CARD_SIZE: LazyLock<Size<Pixels>> = LazyLock::new(|| Size::new(px(480.), px(600.)));
pub const CARD_STEP: f32 = 100.;
pub const SWIPE_THRESHOLD: f32 = 150.0;

// Duration for a single card swipe (manual drag release)
pub const ANIMATION_DURATION: f32 = 0.3; // Default: 0.3

// Base duration for the first card in "Clear All"
pub const CLEAR_ANIMATION_BASE: f32 = 0.5; // Default: 0.5

// Additional delay added for each subsequent card in "Clear All"
pub const CLEAR_ANIMATION_STEP: f32 = 0.3; // Default: 0.1

pub const CLICK_THRESHOLD: f32 = 5.0;

// Duration for initial animation
pub const INITIAL_ANIMATION_DURATION: f32 = 0.2;
