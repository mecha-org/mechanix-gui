use gpui::*;
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

const STATUS_BAR_ICONS_DIR: &str = "icons/status-bar/";

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

// Icon name enum for status icons
#[derive(Clone, Debug)]
enum StatusIconName {
    WirelessOn,
    WirelessOff,
    WirelessHigh,
    WirelessMedium,
    WirelessLow,
    BluetoothOn,
    BluetoothOff,
    BluetoothConnected,
    Battery10,
    Battery20,
    Battery30,
    Battery40,
    Battery50,
    Battery60,
    Battery70,
    Battery80,
    Battery90,
    Battery100,
    BatteryEmpty,
    Battery10Charging,
    Battery20Charging,
    Battery30Charging,
    Battery40Charging,
    Battery50Charging,
    Battery60Charging,
    Battery70Charging,
    Battery80Charging,
    Battery90Charging,
    Battery100Charging,
}

impl StatusIconName {
    fn resolve(&self) -> SharedString {
        let icon_path = match self {
            StatusIconName::WirelessOn => "wireless-on.svg",
            StatusIconName::WirelessOff => "wireless-off.svg",
            StatusIconName::WirelessHigh => "wireless-high.svg",
            StatusIconName::WirelessMedium => "wireless-medium.svg",
            StatusIconName::WirelessLow => "wireless-low.svg",
            StatusIconName::BluetoothOn => "bluetooth-on.svg",
            StatusIconName::BluetoothOff => "bluetooth-off.svg",
            StatusIconName::BluetoothConnected => "bluetooth-connected.svg",
            StatusIconName::Battery10 => "battery-10.svg",
            StatusIconName::Battery20 => "battery-20.svg",
            StatusIconName::Battery30 => "battery-30.svg",
            StatusIconName::Battery40 => "battery-40.svg",
            StatusIconName::Battery50 => "battery-50.svg",
            StatusIconName::Battery60 => "battery-60.svg",
            StatusIconName::Battery70 => "battery-70.svg",
            StatusIconName::Battery80 => "battery-80.svg",
            StatusIconName::Battery90 => "battery-90.svg",
            StatusIconName::Battery100 => "battery-100.svg",
            StatusIconName::BatteryEmpty => "battery-empty.svg",
            StatusIconName::Battery10Charging => "battery-10-charging.svg",
            StatusIconName::Battery20Charging => "battery-20-charging.svg",
            StatusIconName::Battery30Charging => "battery-30-charging.svg",
            StatusIconName::Battery40Charging => "battery-40-charging.svg",
            StatusIconName::Battery50Charging => "battery-50-charging.svg",
            StatusIconName::Battery60Charging => "battery-60-charging.svg",
            StatusIconName::Battery70Charging => "battery-70-charging.svg",
            StatusIconName::Battery80Charging => "battery-80-charging.svg",
            StatusIconName::Battery90Charging => "battery-90-charging.svg",
            StatusIconName::Battery100Charging => "battery-100-charging.svg",
        };
        format!("{}{}", STATUS_BAR_ICONS_DIR, icon_path).into()
    }
}

fn render_wifi_icon(icon: StatusIconName, color: Rgba) -> impl IntoElement {
    svg()
        .path(icon.resolve())
        .w(px(WIFI_ICON_SIZE))
        .h(px(WIFI_ICON_SIZE))
        .text_color(color)
}

fn render_bluetooth_icon(icon: StatusIconName, color: Rgba) -> impl IntoElement {
    svg()
        .path(icon.resolve())
        .w(px(BLUETOOTH_ICON_SIZE))
        .h(px(BLUETOOTH_ICON_SIZE))
        .text_color(color)
}

fn render_battery_icon(icon: StatusIconName, color: Rgba) -> impl IntoElement {
    svg()
        .path(icon.resolve())
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

    // Resolve wireless icon based on state
    let wireless_icon = match wireless_enabled {
        true => match wireless_strength {
            0 => StatusIconName::WirelessOn,
            1..=30 => StatusIconName::WirelessLow,
            31..=60 => StatusIconName::WirelessMedium,
            61..=100 => StatusIconName::WirelessHigh,
            _ => StatusIconName::WirelessOn,
        },
        false => StatusIconName::WirelessOff,
    };

    // Resolve bluetooth icon based on state
    let bluetooth_icon = match bluetooth_enabled {
        true => match bluetooth_connected {
            true => StatusIconName::BluetoothConnected,
            false => StatusIconName::BluetoothOn,
        },
        false => StatusIconName::BluetoothOff,
    };

    // Resolve battery icon based on state and percentage
    let battery_icon = match battery_state {
        BatteryState::Charging => match battery_percent {
            0..=10 => StatusIconName::Battery10Charging,
            11..=20 => StatusIconName::Battery20Charging,
            21..=30 => StatusIconName::Battery30Charging,
            31..=40 => StatusIconName::Battery40Charging,
            41..=50 => StatusIconName::Battery50Charging,
            51..=60 => StatusIconName::Battery60Charging,
            61..=70 => StatusIconName::Battery70Charging,
            71..=80 => StatusIconName::Battery80Charging,
            81..=90 => StatusIconName::Battery90Charging,
            91..=100 => StatusIconName::Battery100Charging,
            _ => StatusIconName::BatteryEmpty,
        },
        BatteryState::Discharging => match battery_percent {
            0..=10 => StatusIconName::Battery10,
            11..=20 => StatusIconName::Battery20,
            21..=30 => StatusIconName::Battery30,
            31..=40 => StatusIconName::Battery40,
            41..=50 => StatusIconName::Battery50,
            51..=60 => StatusIconName::Battery60,
            61..=70 => StatusIconName::Battery70,
            71..=80 => StatusIconName::Battery80,
            81..=90 => StatusIconName::Battery90,
            91..=100 => StatusIconName::Battery100,
            _ => StatusIconName::BatteryEmpty,
        },
        BatteryState::FullCharged => StatusIconName::Battery100,
        BatteryState::Empty => StatusIconName::BatteryEmpty,
        _ => StatusIconName::BatteryEmpty,
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
pub fn left_wedge(colors: &ThemeColors, lock_state: LockState, icon_opacity: f32) -> impl IntoElement {
    let left_wedge_fill_color = colors.accent_200;
    let left_wedge_border_color = colors.accent_300;
    let bell_icon_color = colors.accent_200;
    let bell_circle_color = colors.accent_100.with_alpha(0.1);
    // Left wedge icons remain visible even when unlocked; only hide if opacity is zero.
    let show_icons = icon_opacity > 0.0;

    let mut container = div()
        .absolute()
        .bottom_0()
        .left_0()
        .w(px(LEFT_WEDGE_WIDTH))
        .h(px(LEFT_WEDGE_HEIGHT))
        // Filled wedge background
        .child(
            svg()
                .path("icons/lockscreen/wedge_left.svg")
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
                .path("icons/lockscreen/wedge_left_outline.svg")
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
                                .path("icons/lockscreen/bell.svg")
                                .w(px(BELL_ICON_SIZE))
                                .h(px(BELL_ICON_SIZE))
                                .text_color(bell_icon_color),
                        ),
                )
                // Lock icon in circle (state-based)
                .child(lock_icon(lock_state, colors)),
        );
    }

    container
}

// Right wedge - size 540 x 67, with status icons
pub fn right_wedge(cx: &mut App, colors: &ThemeColors, icon_opacity: f32) -> impl IntoElement {
    let right_wedge_fill_color = colors.accent_200;
    let right_wedge_border_color = colors.accent_300;
    let show_icons = icon_opacity > 0.0;

    let mut container = div()
        .absolute()
        .bottom_0()
        .right_0()
        .w(px(RIGHT_WEDGE_WIDTH))
        .h(px(RIGHT_WEDGE_HEIGHT))
        // Filled wedge background
        .child(
            svg()
                .path("icons/lockscreen/wedge_right.svg")
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
                .path("icons/lockscreen/wedge_right_outline.svg")
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

    fn icon_path(&self) -> &'static str {
        match self {
            LockState::Locked => "icons/lockscreen/lock.svg",
            LockState::HalfOpen => "icons/lockscreen/lock-open-half.svg",
            LockState::FullyOpen => "icons/lockscreen/lock-open-full.svg",
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
pub fn lock_icon(lock_state: LockState, colors: &ThemeColors) -> impl IntoElement {
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
                .path(lock_state.icon_path())
                .w(px(LOCK_ICON_SIZE))
                .h(px(LOCK_ICON_SIZE))
                .text_color(lock_state.icon_color(colors)),
        )
}
