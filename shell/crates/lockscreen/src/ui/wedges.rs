use gpui::*;
use shell_state::ShellState;
use upower::interfaces::device::BatteryState;

pub const STATUS_ICON_GAP: f32 = 15.0;
pub const STATUS_ICON_PADDING_RIGHT: f32 = 20.0;
pub const STATUS_ICON_PADDING_BOTTOM: f32 = 30.0;
pub const STATUS_ICON_COLOR: u32 = 0xFFFFFFFF; 

// Individual icon sizes - customize each independently
pub const WIFI_ICON_SIZE: f32 = 25.0;
pub const BLUETOOTH_ICON_SIZE: f32 = 25.0;
pub const BATTERY_ICON_SIZE: f32 = 30.0;

const STATUS_BAR_ICONS_DIR: &str = "icons/status-bar/";

// Left wedge dimensions: 540 x 106 (from wedge_left.svg )
pub const LEFT_WEDGE_WIDTH: f32 = 540.0;
pub const LEFT_WEDGE_HEIGHT: f32 = 106.0;

// Right wedge dimensions: 540 x 67 (from wedge_right.svg)
pub const RIGHT_WEDGE_WIDTH: f32 = 540.0;
pub const RIGHT_WEDGE_HEIGHT: f32 = 67.0;

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
fn status_icons(cx: &mut App) -> impl IntoElement {
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

    // Icon color from constant
    let icon_color = rgba(STATUS_ICON_COLOR);

    div()
        .flex()
        .flex_row()
        .gap(px(STATUS_ICON_GAP))
        .child(render_wifi_icon(wireless_icon, icon_color))
        .child(render_bluetooth_icon(bluetooth_icon, icon_color))
        .child(render_battery_icon(battery_icon, icon_color))
}

// Left wedge - size 540 x 106, color #382000
pub fn left_wedge(content: impl IntoElement) -> impl IntoElement {
    let svg_color = rgba(0x382000FF);

    div()
        .absolute()
        .bottom_0()
        .left_0()
        .w(px(LEFT_WEDGE_WIDTH))
        .h(px(LEFT_WEDGE_HEIGHT))
        .child(
            svg()
                .path("icons/lockscreen/wedge_left.svg")
                .absolute()
                .inset_0()
                .w(px(LEFT_WEDGE_WIDTH))
                .h(px(LEFT_WEDGE_HEIGHT))
                .text_color(svg_color),
        )
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(content),
        )
}

// Right wedge - size 540 x 67, color #1F1200, with status icons
pub fn right_wedge(cx: &mut App) -> impl IntoElement {
    let svg_color = rgba(0x1F1200FF);

    div()
        .absolute()
        .bottom_0()
        .right_0()
        .w(px(RIGHT_WEDGE_WIDTH))
        .h(px(RIGHT_WEDGE_HEIGHT))
        .child(
            svg()
                .path("icons/lockscreen/wedge_right.svg")
                .absolute()
                .inset_0()
                .w(px(RIGHT_WEDGE_WIDTH))
                .h(px(RIGHT_WEDGE_HEIGHT))
                .text_color(svg_color),
        )
        .child(
            div()
                .absolute()
                .bottom(px(STATUS_ICON_PADDING_BOTTOM))
                .right(px(STATUS_ICON_PADDING_RIGHT))
                .flex()
                .items_center()
                .child(status_icons(cx)),
        )
}
