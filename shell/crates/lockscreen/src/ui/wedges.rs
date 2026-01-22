use std::path::PathBuf;

use gpui::*;
use icons::prelude::Icons;
use shell_state::ShellState;
use theme::prelude::{AlphaExt, ThemeColors};
use upower::interfaces::device::BatteryState;

pub const STATUS_ICON_GAP: f32 = 15.0;
pub const STATUS_ICON_PADDING_RIGHT: f32 = 20.0;
pub const STATUS_ICON_PADDING_BOTTOM: f32 = 28.0;

// Individual icon sizes - customize each independently
pub const WIFI_ICON_SIZE: f32 = 25.0;
pub const BLUETOOTH_ICON_SIZE: f32 = 25.0;
pub const BATTERY_ICON_SIZE: f32 = 30.0;

// Bell icon configuration (left wedge)
pub const BELL_ICON_SIZE: f32 = 22.0;
pub const BELL_CIRCLE_SIZE: f32 = 40.0;
pub const BELL_PADDING_LEFT: f32 = 12.0;
pub const BELL_PADDING_BOTTOM: f32 = 56.0;

// Lock icon configuration (center, above panel)
pub const LOCK_ICON_SIZE: f32 = 24.0;
pub const LOCK_CIRCLE_SIZE: f32 = 40.0;

// Left wedge dimensions: 540 x 106 (from wedge_left.svg )
pub const LEFT_WEDGE_WIDTH: f32 = 540.0;
pub const LEFT_WEDGE_HEIGHT: f32 = 106.0;

// Right wedge dimensions: 540 x 67 (from wedge_right.svg)
pub const RIGHT_WEDGE_WIDTH: f32 = 540.0;
pub const RIGHT_WEDGE_HEIGHT: f32 = 67.0;

// Wedge border/outline configuration
pub const LEFT_WEDGE_BORDER_THICKNESS: f32 = 2.0;
pub const RIGHT_WEDGE_BORDER_THICKNESS: f32 = 2.0;

// Gap between bell and lock icons in left wedge
pub const LEFT_WEDGE_ICON_GAP: f32 = 12.0;

fn render_wifi_icon(icon: PathBuf, color: Rgba) -> impl IntoElement {
    svg()
        .external_path(SharedString::from(icon.to_string_lossy().to_string()))
        .w(px(WIFI_ICON_SIZE))
        .h(px(WIFI_ICON_SIZE))
        .text_color(color)
}

fn render_bluetooth_icon(icon: PathBuf, color: Rgba) -> impl IntoElement {
    svg()
        .external_path(SharedString::from(icon.to_string_lossy().to_string()))
        .w(px(BLUETOOTH_ICON_SIZE))
        .h(px(BLUETOOTH_ICON_SIZE))
        .text_color(color)
}

fn render_battery_icon(icon: PathBuf, color: Rgba) -> impl IntoElement {
    svg()
        .external_path(SharedString::from(icon.to_string_lossy().to_string()))
        .w(px(BATTERY_ICON_SIZE))
        .h(px(BATTERY_ICON_SIZE))
        .text_color(color)
}

/// Creates the status icons (wifi, bluetooth, battery) based on current ShellState
fn status_icons(cx: &mut App, colors: &ThemeColors) -> impl IntoElement {
    let ShellState {
        wireless_details,
        bluetooth_details,
        battery_state,
        battery_percent,
        ..
    } = ShellState::global(cx);

    let wireless_enabled = wireless_details.enabled;
    let bluetooth_enabled = bluetooth_details.enabled;
    let bluetooth_connected = bluetooth_details.connected_devices > 0;
    let wireless_strength = wireless_details
        .connected_network
        .as_ref()
        .map_or(0, |n| n.signal_strength);

    let icons = Icons::global(cx).status_bar.clone();

    // Resolve wireless icon based on state
    let wireless_icon = match wireless_enabled {
        true => match wireless_strength {
            0 => icons.wireless_on,
            1..=30 => icons.wireless_low,
            31..=60 => icons.wireless_medium,
            61..=100 => icons.wireless_high,
            _ => icons.wireless_on,
        },
        false => icons.wireless_off,
    };

    // Resolve bluetooth icon based on state
    let bluetooth_icon = match bluetooth_enabled {
        true => match bluetooth_connected {
            true => icons.bluetooth_connected,
            false => icons.bluetooth_on,
        },
        false => icons.bluetooth_off,
    };

    // Resolve battery icon based on state and percentage
    let battery_icon = match battery_state {
        BatteryState::Charging => match battery_percent {
            0..=10 => icons.battery_10_charging,
            11..=20 => icons.battery_20_charging,
            21..=30 => icons.battery_30_charging,
            31..=40 => icons.battery_40_charging,
            41..=50 => icons.battery_50_charging,
            51..=60 => icons.battery_60_charging,
            61..=70 => icons.battery_70_charging,
            71..=80 => icons.battery_80_charging,
            81..=90 => icons.battery_90_charging,
            91..=100 => icons.battery_100_charging,
            _ => icons.battery_empty,
        },
        BatteryState::Discharging => match battery_percent {
            0..=10 => icons.battery_10,
            11..=20 => icons.battery_20,
            21..=30 => icons.battery_30,
            31..=40 => icons.battery_40,
            41..=50 => icons.battery_50,
            51..=60 => icons.battery_60,
            61..=70 => icons.battery_70,
            71..=80 => icons.battery_80,
            81..=90 => icons.battery_90,
            91..=100 => icons.battery_100,
            _ => icons.battery_empty,
        },
        BatteryState::FullCharged => icons.battery_100,
        BatteryState::Empty => icons.battery_empty,
        _ => icons.battery_empty,
    };

    // Icon color from theme accent ramp
    let status_icon_color = colors.accent_200;

    div()
        .flex()
        .flex_row()
        .gap(px(STATUS_ICON_GAP))
        .child(render_wifi_icon(wireless_icon, status_icon_color))
        .child(render_bluetooth_icon(bluetooth_icon, status_icon_color))
        .child(render_battery_icon(battery_icon, status_icon_color))
}

// Left wedge - size 540 x 106, with bell icon and lock icon
pub fn left_wedge(
    colors: &ThemeColors,
    lock_state: LockState,
    icon_opacity: f32,
    cx: &mut App,
) -> impl IntoElement {
    let left_wedge_fill_color = colors.accent_200;
    let left_wedge_border_color = colors.accent_300;
    let bell_icon_color = colors.accent_200;
    let bell_circle_color = colors.accent_100.with_alpha(0.1);
    // Left wedge icons remain visible even when unlocked; only hide if opacity is zero.
    let show_icons = icon_opacity > 0.0;
    let icons = Icons::global(cx).lockscreen.clone();

    let mut container = div()
        .absolute()
        .bottom_0()
        .left_0()
        .w(px(LEFT_WEDGE_WIDTH))
        .h(px(LEFT_WEDGE_HEIGHT))
        // Filled wedge background
        .child(
            svg()
                .external_path(SharedString::from(
                    icons.wedge_left.to_string_lossy().to_string(),
                ))
                .absolute()
                .opacity(0.20)
                .inset_0()
                .w(px(LEFT_WEDGE_WIDTH))
                .h(px(LEFT_WEDGE_HEIGHT))
                .text_color(left_wedge_fill_color),
        )
        // Outline overlay
        .child(
            svg()
                .external_path(SharedString::from(
                    icons.wedge_left_outline.to_string_lossy().to_string(),
                ))
                .absolute()
                .inset_0()
                .opacity(0.80)
                .w(px(LEFT_WEDGE_WIDTH))
                .h(px(LEFT_WEDGE_HEIGHT))
                .text_color(left_wedge_border_color),
        );

    if show_icons {
        container = container.child(
            div()
                .absolute()
                .bottom(px(BELL_PADDING_BOTTOM))
                .left(px(BELL_PADDING_LEFT))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(LEFT_WEDGE_ICON_GAP))
                .opacity(icon_opacity.max(0.0))
                // Bell icon in circle
                .child(
                    div()
                        .w(px(BELL_CIRCLE_SIZE))
                        .h(px(BELL_CIRCLE_SIZE))
                        .rounded_full()
                        .bg(bell_circle_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            svg()
                                .external_path(SharedString::from(
                                    icons.bell.to_string_lossy().to_string(),
                                ))
                                .w(px(BELL_ICON_SIZE))
                                .h(px(BELL_ICON_SIZE))
                                .text_color(bell_icon_color),
                        ),
                )
                // Lock icon in circle (state-based)
                .child(lock_icon(lock_state, colors, cx)),
        );
    }

    container
}

// Right wedge - size 540 x 67, with status icons
pub fn right_wedge(colors: &ThemeColors, icon_opacity: f32, cx: &mut App) -> impl IntoElement {
    let right_wedge_fill_color = colors.accent_200;
    let right_wedge_border_color = colors.accent_300;
    let show_icons = icon_opacity > 0.0;
    let icons = Icons::global(cx).lockscreen.clone();

    let mut container = div()
        .absolute()
        .bottom_0()
        .right_0()
        .w(px(RIGHT_WEDGE_WIDTH))
        .h(px(RIGHT_WEDGE_HEIGHT))
        // Filled wedge background
        .child(
            svg()
                .external_path(SharedString::from(
                    icons.wedge_right.to_string_lossy().to_string(),
                ))
                .absolute()
                .inset_0()
                .w(px(RIGHT_WEDGE_WIDTH))
                .h(px(RIGHT_WEDGE_HEIGHT))
                .opacity(0.20)
                .text_color(right_wedge_fill_color),
        )
        // Outline overlay
        .child(
            svg()
                .external_path(SharedString::from(
                    icons.wedge_right_outline.to_string_lossy().to_string(),
                ))
                .absolute()
                .inset_0()
                .opacity(0.80)
                .w(px(RIGHT_WEDGE_WIDTH))
                .h(px(RIGHT_WEDGE_HEIGHT))
                .text_color(right_wedge_border_color),
        );

    if show_icons {
        container = container.child(
            div()
                .absolute()
                .bottom(px(STATUS_ICON_PADDING_BOTTOM))
                .right(px(STATUS_ICON_PADDING_RIGHT))
                .flex()
                .items_center()
                .opacity(icon_opacity.max(0.0))
                .child(status_icons(cx, colors)),
        );
    }

    container
}

/// Lock icon state based on slider position
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LockState {
    Locked,    // At rest (position_y = 0)
    HalfOpen,  // Dragging but below threshold
    FullyOpen, // Beyond unlock threshold
}

impl LockState {
    /// Determine lock state from position_y and threshold
    pub fn from_position(position_y: f32, threshold: f32) -> Self {
        if position_y >= 0.0 {
            LockState::Locked
        } else if position_y > -threshold {
            LockState::HalfOpen
        } else {
            LockState::FullyOpen
        }
    }

    fn icon_path(&self, cx: &mut App) -> PathBuf {
        let icons = Icons::global(cx).lockscreen.clone();

        match self {
            LockState::Locked => icons.lock,
            LockState::HalfOpen => icons.lock_open_half,
            LockState::FullyOpen => icons.lock_open_full,
        }
    }

    fn icon_color(&self, colors: &ThemeColors) -> Rgba {
        let _ = self; // state currently shares the same accent color
        colors.accent_200
    }

    fn bg_color(&self, colors: &ThemeColors) -> Rgba {
        match self {
            LockState::Locked => colors.accent_100.with_alpha(0.1),
            LockState::HalfOpen => colors.accent_100.with_alpha(0.5),
            LockState::FullyOpen => colors.accent_100.with_alpha(0.7),
        }
    }
}

/// Lock icon that changes based on slider position
/// - Locked: at rest (position_y = 0)
/// - HalfOpen: dragging but below threshold
/// - FullyOpen: beyond unlock threshold
pub fn lock_icon(lock_state: LockState, colors: &ThemeColors, cx: &mut App) -> impl IntoElement {
    div()
        .w(px(LOCK_CIRCLE_SIZE))
        .h(px(LOCK_CIRCLE_SIZE))
        .rounded_full()
        .bg(lock_state.bg_color(colors))
        .flex()
        .items_center()
        .justify_center()
        .child(
            svg()
                .external_path(SharedString::from(
                    lock_state.icon_path(cx).to_string_lossy().to_string(),
                ))
                .w(px(LOCK_ICON_SIZE))
                .h(px(LOCK_ICON_SIZE))
                .text_color(lock_state.icon_color(colors)),
        )
}
