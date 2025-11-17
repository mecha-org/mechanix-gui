mod icon;
mod widgets;
use crate::ui::icon::Icon;
use gpui::*;
use icon::IconName;
use widgets::IconButton;

pub enum PowerMode {
    High,
    Balanced,
    Low,
}

pub struct WirelessDetails {
    pub enalble: bool,
    pub icon: IconName,
    pub connected_wifi: Option<String>,
}

pub struct BluetoothDetails {
    pub enalble: bool,
    pub icon: IconName,
    pub connected_device: Option<String>,
}

pub struct SettingsDrawer {
    pub settings_active: bool,
    pub battery_percent: u8,
    pub open_power_options: bool,

    pub rotation_on: bool,
    pub airplane_mode: bool,
    pub screen_mirrorring: bool,
    pub power_mode: PowerMode,
    pub mincrophone_recoding: bool,
    pub screen_recording: bool,
    pub calc_active: bool,
    pub camera_active: bool,

    pub wireless_details: WirelessDetails,
    pub bluetooth_details: BluetoothDetails,
    pub open_terminal: bool,
    pub cell_signal: bool,
}

impl SettingsDrawer {
    pub fn new() -> Self {
        Self {
            settings_active: false,
            battery_percent: 32,
            open_power_options: false,
            rotation_on: false,
            airplane_mode: false,
            screen_mirrorring: false,
            power_mode: PowerMode::Low,
            mincrophone_recoding: false,
            screen_recording: false,
            calc_active: true,
            camera_active: false,
            wireless_details: WirelessDetails {
                enalble: true,
                icon: IconName::WirelessHigh,
                connected_wifi: Some("Office Wifi 1".to_string()),
            },
            bluetooth_details: BluetoothDetails {
                enalble: false,
                icon: IconName::BluetoothOff,
                connected_device: None,
            },
            open_terminal: false,
            cell_signal: false,
        }
    }
}

impl Render for SettingsDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rotation_icon = if self.rotation_on {
            IconName::RotationOn
        } else {
            IconName::RotationOff
        };

        let screen_mirroring_icon = if self.screen_mirrorring {
            IconName::ScreenMirroringOn
        } else {
            IconName::ScreenMirroringOff
        };
        let power_mode_icon = match self.power_mode {
            PowerMode::High => IconName::PowerModeHigh,
            PowerMode::Balanced => IconName::PowerModeBalanced,
            PowerMode::Low => IconName::PowerModeLow,
        };
        let power_mode_icon_color = match self.power_mode {
            PowerMode::High => rgb(0x4892F1),     // blue
            PowerMode::Balanced => rgb(0x4D4D4D), // gray
            PowerMode::Low => rgb(0xEBB503),      // yellow
        };

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x101010))
            .pl_8()
            .pr_8()
            .w_full()
            .h_full()
            .content_stretch()
            .gap_4()
            .pt(px(1.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h(px(24.))
                    .items_end()
                    .justify_end()
                    .child(img("icons/settings-drawer/right_nav_bar.png")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h(px(32.82))
                    .items_center()
                    .justify_between()
                    .child(
                        IconButton::new("id_settings")
                            .icon(IconName::Settings)
                            .icon_color(rgb(0xF4F4F4))
                            .size((px(24.), px(24.)))
                            .border(px(0.))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("settings clicked");
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .child(
                                div()
                                    .text_lg()
                                    .text_color(rgb(0xE9E9E9))
                                    .child(format!("{}% ", self.battery_percent)),
                            )
                            .child(
                                Icon::new(IconName::Battery)
                                    .size((px(20.), px(20.)))
                                    .text_color(rgb(0xE9E9E9)),
                            ),
                    )
                    .child(
                        IconButton::new("id_power")
                            .icon(IconName::Power)
                            .icon_color(rgb(0xF4F4F4))
                            .size((px(24.), px(24.)))
                            .border(px(0.))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("power clicked");
                            })),
                    ),
            )
            .child(
                div()
                    .grid()
                    .grid_rows(2)
                    .grid_cols(4)
                    .h(px(236.))
                    .bg(rgb(0x181818))
                    .p_6()
                    .gap_5()
                    .rounded(px(12.))
                    .child(
                        IconButton::new("id_rotation")
                            .icon(rotation_icon)
                            .active(self.rotation_on)
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.rotation_on = !this.rotation_on;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_airplane")
                            .icon(IconName::Airplane)
                            .icon_color(rgb(0xF4F4F4))
                            .active(self.airplane_mode)
                            .active_icon_color(rgb(0xF4F4F4))
                            .active_bg_color(rgb(0xDB9200))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("airplane button clicked");
                                    this.airplane_mode = !this.airplane_mode;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_screen_mirroring")
                            .icon(screen_mirroring_icon)
                            .active(self.screen_mirrorring)
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.screen_mirrorring = !this.screen_mirrorring;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_power_mode")
                            .icon(power_mode_icon)
                            .icon_color(power_mode_icon_color)
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("power mode clicked--open modal!");
                            })),
                    )
                    .child(
                        IconButton::new("id_microphone")
                            .icon(IconName::MicroPhoneOff)
                            .active(self.mincrophone_recoding)
                            .active_icon_color(rgb(0xFF6560))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.mincrophone_recoding = !this.mincrophone_recoding;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_screen_recording")
                            .icon(IconName::ScreenRecordingOff)
                            .active(self.screen_recording)
                            .active_icon_color(rgb(0xFF6560))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.screen_recording = !this.screen_recording;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_calc")
                            .icon(IconName::Calculator)
                            .icon_color(rgb(0xF4F4F4))
                            .active(self.calc_active)
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("calc clicked");
                                    this.calc_active = !this.calc_active;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_camera")
                            .icon(IconName::Camera)
                            .icon_color(rgb(0xF4F4F4))
                            .active(self.camera_active)
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("camera clicked");
                                    this.camera_active = !this.camera_active;
                                    cx.notify();
                                },
                            )),
                    ),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(4)
                    .gap_4()
                    .h(px(104.))
                    .rounded(px(12.))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_full()
                            .h_full()
                            .text_color(rgb(0xF4F4F4))
                            .text_lg()
                            .col_span(2)
                            .bg(rgb(0x202020))
                            .rounded(px(12.))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .items_center()
                                    .p_2()
                                    .child(
                                        IconButton::new("id_brightness")
                                            .icon(IconName::BrightnessHigh)
                                            .icon_color(rgb(0xF4F4F4))
                                            .size((px(36.), px(36.)))
                                            .border(px(0.))
                                            .on_click(cx.listener(|_, _, _, _| {
                                                println!("brightness clicked");
                                            })),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_center()
                                            .items_center()
                                            .w(px(172.0))
                                            .h(px(72.0))
                                            .bg(rgb(0x202020))
                                            .rounded(px(12.))
                                            .child("sliderrrrr--------rrrrr"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_full()
                            .h_full()
                            .text_color(rgb(0xF4F4F4))
                            .text_lg()
                            .col_span(2)
                            .bg(rgb(0x202020))
                            .rounded(px(12.))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .items_center()
                                    .p_2()
                                    .child(
                                        IconButton::new("id_volume")
                                            .icon(IconName::VolumeMedium)
                                            .icon_color(rgb(0xF4F4F4))
                                            .size((px(36.), px(36.)))
                                            .border(px(0.))
                                            .on_click(cx.listener(|_, _, _, _| {
                                                println!("volume clicked");
                                            })),
                                    )
                                    .child("sliderrrrr--------rrrrr"),
                            ),
                    ),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(4)
                    .gap_5()
                    .h(px(104.))
                    .rounded(px(4.))
                    .bg(rgb(0x181818))
                    .child(
                        IconButton::new("id_wireless")
                            .icon(self.wireless_details.icon.clone())
                            .icon_color(rgb(0x4D4D4D))  // changes as per wireless state
                            .size((px(104.), px(104.)))
                            .active(self.wireless_details.enalble)
                            .label("Office wifi 1")
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("wireless clicked");
                            })),
                    )
                    .child(
                        IconButton::new("id_bluetooth")
                            .icon(self.bluetooth_details.icon.clone())
                            .size((px(104.), px(104.)))
                            .label("OFF")
                            .active(self.bluetooth_details.enalble)
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("bluetooth clicked");
                            })),
                    )
                    .child(
                        IconButton::new("id_terminal")
                            .icon(IconName::Terminal)
                            .size((px(104.), px(104.)))
                            .label("Terminal")
                            .icon_color(rgb(0xF4F4F4))
                            .active(self.open_terminal)
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("terminal clicked");
                                    this.open_terminal = !this.open_terminal;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_cell_signal")
                            .icon(IconName::CellSignalNone)
                            .size((px(104.), px(104.)))
                            .label("No SIM")
                            .active(self.cell_signal)
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("cell_signal clicked");
                                    this.cell_signal = !this.cell_signal;
                                    cx.notify();
                                },
                            )),
                    ),
            )
    }
}
