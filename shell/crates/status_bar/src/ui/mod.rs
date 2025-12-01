use chrono::Local;
use gpui::*;
pub mod icon;
pub use icon::{Icon, IconName};
use upower::interfaces::device::BatteryState;

pub fn get_current_datetime() -> String {
    let now = Local::now();
    format!("{}", now.format("%H:%M"))
}

pub struct StatusBar {
    pub current_time_date: String,
    pub wireless_enabled: bool,
    pub wireless_strength: u8,
    pub bluetooth_enabled: bool,
    pub bluetooth_connected: bool,
    pub battery_state: BatteryState,
    pub battery_percent: u8,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            current_time_date: get_current_datetime(),
            wireless_enabled: false,
            wireless_strength: 0,
            bluetooth_enabled: false,
            bluetooth_connected: false,
            battery_state: BatteryState::Unknown,
            battery_percent: 0,
        }
    }

    pub fn update_time(&mut self, cx: &mut Context<Self>) {
        self.current_time_date = get_current_datetime();
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {

        let wireless_icon = match self.wireless_enabled {
            true => match self.wireless_strength {
                0 => IconName::WirelessOn,
                1..=30 => IconName::WirelessLow,
                31..=60 => IconName::WirelessMedium,
                61..=100 => IconName::WirelessHigh,
                _ => IconName::WirelessWarning,
            },
            false => IconName::WirelessOff,
        };

        let bluetooth_icon = match self.bluetooth_enabled {
            true => match self.bluetooth_connected {
                true => IconName::BluetoothConnected,
                false => IconName::BluetoothOn,
            },
            false => IconName::BluetoothOff,
        };

        let battery_icon = match self.battery_state {
            BatteryState::Charging => match self.battery_percent {
                0..=10 => IconName::Battery10Charging,
                11..=20 => IconName::Battery20Charging,
                21..=30 => IconName::Battery30Charging,
                31..=40 => IconName::Battery40Charging,
                41..=50 => IconName::Battery50Charging,
                51..=60 => IconName::Battery60Charging,
                61..=70 => IconName::Battery70Charging,
                71..=80 => IconName::Battery80Charging,
                81..=90 => IconName::Battery90Charging,
                91..=100 => IconName::Battery100Charging,
                _ => IconName::BatteryEmpty,
            },
            BatteryState::Discharging => match self.battery_percent {
                0..=10 => IconName::Battery10,
                11..=20 => IconName::Battery20,
                21..=30 => IconName::Battery30,
                31..=40 => IconName::Battery40,
                41..=50 => IconName::Battery50,
                51..=60 => IconName::Battery60,
                61..=70 => IconName::Battery70,
                71..=80 => IconName::Battery80,
                81..=90 => IconName::Battery90,
                91..=100 => IconName::Battery100,
                _ => IconName::BatteryEmpty,
            },
            BatteryState::FullCharged => IconName::Battery100,
            BatteryState::Empty => IconName::BatteryEmpty,
            _ => IconName::BatteryEmpty,
        };

        div()
            .bg(gpui::transparent_white())
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_full()
            .text_color(gpui::white())
            .pl_4()
            .pr_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .child(self.current_time_date.clone())
                    .text_lg()
                    .text_color(rgb(0xE9E9E9)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .items_end()
                    .child(
                        Icon::new(wireless_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    )
                    .child(
                        Icon::new(bluetooth_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    )
                    .child(
                        Icon::from(battery_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    ),
            )
    }
}
