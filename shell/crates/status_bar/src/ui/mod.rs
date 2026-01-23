use gpui::{prelude::FluentBuilder, *};
use icons::prelude::{Icons, StatusBarIcons};
use settings::prelude::Settings;
use shell_state::ShellState;
use std::time::Duration;
use theme::prelude::{AlphaExt, Fonts, Theme};
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
        let colors = Theme::global(cx).colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();

        div()
            .id("status-bar")
            .w(px(1.))
            .h(px(1.))
            .bg(colors.background_1000)
            .when(window.foreign_toplevels().len() > 0, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .bg(colors.background_1000.with_alpha(0.0))
                        .flex()
                        .justify_between()
                        .w(size.width)
                        .h(size.height)
                        .text_color(colors.foreground_0)
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
    let colors = Theme::global(cx).colors.clone();
    let primary_font = Fonts::global(cx).primary.clone();

    let StatusBarIcons {
        battery_0_charging,
        battery_10_charging,
        battery_10,
        battery_100_charging,
        battery_100,
        battery_20_charging,
        battery_20,
        battery_30_charging,
        battery_30,
        battery_40_charging,
        battery_40,
        battery_50_charging,
        battery_50,
        battery_60_charging,
        battery_60,
        battery_70_charging,
        battery_70,
        battery_80_charging,
        battery_80,
        battery_90_charging,
        battery_90,
        battery_empty,
        bluetooth_connected,
        bluetooth_off,
        bluetooth_on,
        bluetooth_warning,
        wireless_high,
        wireless_low,
        wireless_medium,
        wireless_off,
        wireless_on,
        wireless_warning,
        ..
    } = Icons::global(cx).status_bar.clone();

    let wireless_enabled = wireless_details.enabled;
    let bluetooth_enabled = bluetooth_details.enabled;
    let is_bluetooth_connected = bluetooth_details.connected_devices > 0;
    let wireless_strength = wireless_details
        .connected_network
        .as_ref()
        .map_or(0, |n| n.signal_strength);

    let wireless_icon_path = match wireless_enabled {
        true => match wireless_strength {
            0 => wireless_on,
            1..=30 => wireless_low,
            31..=60 => wireless_medium,
            61..=100 => wireless_high,
            _ => wireless_warning,
        },
        false => wireless_off,
    };

    let bluetooth_icon_path = match bluetooth_enabled {
        true => match is_bluetooth_connected {
            true => bluetooth_connected,
            false => bluetooth_on,
        },
        false => bluetooth_off,
    };

    let battery_icon_path = match battery_state {
        BatteryState::Charging => match battery_percent {
            0 => battery_0_charging,
            1..=10 => battery_10_charging,
            11..=20 => battery_20_charging,
            21..=30 => battery_30_charging,
            31..=40 => battery_40_charging,
            41..=50 => battery_50_charging,
            51..=60 => battery_60_charging,
            61..=70 => battery_70_charging,
            71..=80 => battery_80_charging,
            81..=90 => battery_90_charging,
            91..=100 => battery_100_charging,
            _ => battery_empty,
        },

        BatteryState::Discharging => match battery_percent {
            0 => battery_empty,
            1..=10 => battery_10,
            11..=20 => battery_20,
            21..=30 => battery_30,
            31..=40 => battery_40,
            41..=50 => battery_50,
            51..=60 => battery_60,
            61..=70 => battery_70,
            71..=80 => battery_80,
            81..=90 => battery_90,
            91..=100 => battery_100,
            _ => battery_empty,
        },
        BatteryState::FullCharged => battery_100,
        BatteryState::Empty => battery_empty,
        _ => battery_empty,
    };
    let time_format = Settings::global(cx).homescreen.time_format.clone();
    let current_time = format!("{}", current_time_date.format(time_format.as_str()));

    div()
        .when_else(
            !is_homescreen,
            |this| {
                this.absolute().top(px(0.)).left(px(0.)).bg(linear_gradient(
                    0.,
                    linear_color_stop(colors.background_1000.with_alpha(1.), 1.),
                    linear_color_stop(colors.background_1000.with_alpha(0.), 0.2),
                )
                .color_space(ColorSpace::default()))
            },
            |this| this.bg(colors.background_1000),
        )
        .flex()
        .justify_between()
        .items_center()
        .w(size.width)
        .h(size.height)
        .px_4()
        .child(
            div()
                .flex()
                .flex_row()
                .child(current_time)
                .font_family(primary_font)
                .font_weight(FontWeight::NORMAL)
                .line_height(px(1.2))
                .text_color(colors.foreground_200)
                .text_size(px(16.)),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(img(wireless_icon_path).w(px(20.)).h(px(20.)))
                .when(bluetooth_enabled, |this_div| {
                    this_div.child(img(bluetooth_icon_path).w(px(20.)).h(px(20.)))
                })
                .child(img(battery_icon_path).w(px(20.)).h(px(20.))),
        )
}
