mod icon;
use gpui::*;
use icon::IconName;

use crate::ui::icon::Icon;
use chrono::{Datelike, Timelike};

pub fn get_current_datetime() -> String {
    let now = chrono::Local::now();
    format!(
        "{} {} {:02}:{:02}",
        now.day(),
        now.format("%B"),
        now.hour(),
        now.minute(),
    )
}

pub struct StatusBar {
    pub current_time_date: String,
    pub wireless_default_icon: IconName,
    pub bluetooth_default_icon: IconName,
    pub battery_default_icon: IconName,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            current_time_date: get_current_datetime(),
            wireless_default_icon: IconName::WirelessHigh,
            bluetooth_default_icon: IconName::BluetoothOn,
            battery_default_icon: IconName::Battery100,
        }
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::black())
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
                        Icon::new(self.wireless_default_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    )
                    .child(
                        Icon::new(self.bluetooth_default_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    )
                    .child(
                        Icon::from(self.battery_default_icon.clone())
                            .size((px(20.0), px(20.0)))
                            .text_color(rgb(0xE9E9E9)),
                    ),
            )
    }
}
