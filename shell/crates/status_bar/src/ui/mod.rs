use gpui::{prelude::FluentBuilder, *};
use std::time::Duration;
pub mod icon;
pub use icon::{Icon, IconName};
use shell_state::ShellState;
use upower::interfaces::device::{BatteryLevel, BatteryState};

pub struct StatusBar {
    pub wireless_connected: bool,
    pub bluetooth_connected: bool,
    pub battery_state: BatteryState,
    pub battery_level: BatteryLevel,
    pub drag_offset: f32,
    pub drag_start_pos: Option<f32>,
    pub show: bool,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            wireless_connected: false,
            bluetooth_connected: false,
            battery_state: BatteryState::Unknown,
            battery_level: BatteryLevel::None,
            drag_offset: 0.,
            drag_start_pos: None,
            show: false,
        }
    }

    pub fn show_status_bar(&mut self, cx: &mut Context<Self>) {
        //if user is already dragging then return
        if self.drag_start_pos.is_some() {
            return;
        }
        self.show = true;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(2)).await;
            let _ = this.update(cx, |this, cx| {
                this.show = false;
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.bounds().size;

        div()
            .id("status-bar")
            .w(px(1.))
            .h(px(1.))
            .bg(gpui::black())
            .when(window.foreign_toplevels().len() > 0, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .bg(gpui::transparent_black())
                        .flex()
                        .justify_between()
                        .w(size.width)
                        .h(size.height)
                        .text_color(gpui::white())
                        .pt_2()
                        .pl_4()
                        .pr_4()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, window, cx| {
                                cx.stop_propagation();
                                this.drag_start_pos = Some(event.position.y.to_f64() as f32);
                                cx.notify();
                            }),
                        )
                        .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, window, cx| {
                            cx.stop_propagation();
                            if let Some(drag_start_pos) = this.drag_start_pos {
                                if (event.position.y.to_f64() as f32 - drag_start_pos) > 10. {
                                    this.show = true;
                                } else {
                                    this.show = false;
                                }
                                cx.notify();
                            }
                        }))
                        .on_mouse_up(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.drag_start_pos = None;
                                if this.show {
                                    this.show_status_bar(cx)
                                }
                            }),
                        ),
                )
            })
            .when(self.show && window.foreign_toplevels().len() > 0, |this| {
                this.child(status_bar_components(cx, size, false))
            })
    }
}

pub fn status_bar_components(
    cx: &mut App,
    size: Size<Pixels>,
    is_homescreen: bool,
) -> impl IntoElement {
    let ShellState {
        current_time_date,
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

    let wireless_icon = match wireless_enabled {
        true => match wireless_strength {
            0 => IconName::WirelessOn,
            1..=30 => IconName::WirelessLow,
            31..=60 => IconName::WirelessMedium,
            61..=100 => IconName::WirelessHigh,
            _ => IconName::WirelessWarning,
        },
        false => IconName::WirelessOff,
    };

    let bluetooth_icon = match bluetooth_enabled {
        true => match bluetooth_connected {
            true => IconName::BluetoothConnected,
            false => IconName::BluetoothOn,
        },
        false => IconName::BluetoothOff,
    };

    let battery_icon = match battery_state {
        BatteryState::Charging => match battery_percent {
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
        BatteryState::Discharging => match battery_percent {
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
        .when_else(
            !is_homescreen,
            |this| {
                this.absolute().top(px(0.)).left(px(0.)).bg(linear_gradient(
                    0.,
                    linear_color_stop(
                        Rgba {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 1.,
                        },
                        0.3,
                    ),
                    linear_color_stop(
                        Rgba {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 0.0,
                        },
                        0.5,
                    ),
                )
                .color_space(ColorSpace::default()))
            },
            |this| this.bg(gpui::black()),
        )
        .flex()
        .justify_between()
        .w(size.width)
        .h(size.height)
        .text_color(gpui::white())
        .pt_2()
        .pl_4()
        .pr_4()
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .child(current_time_date.clone())
                .text_lg()
                .text_color(rgb(0xE9E9E9)),
        )
        .child(
            div()
                .flex()
                .gap_2()
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
