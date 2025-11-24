mod icon;
mod widgets;
use crate::events::NmEvents;
use crate::{
    events::BtEvents,
    ui::{
        icon::{Icon, IconName},
        widgets::{IconButton, Slider, SliderEvent, SliderState},
    },
};
use futures::{SinkExt, channel::mpsc};
use gpui::*;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;
use pulseaudio::service::DeviceInfo;

pub enum PowerMode {
    High,
    Balanced,
    Low,
}

pub struct WirelessDetails {
    pub enabled: bool,
    pub strength: u8,
    pub connected_network: Option<WirelessNetworkInfo>,
}

pub struct BluetoothDetails {
    pub enabled: bool,
    pub devices: u8,
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

    pub wireless_details: WirelessDetails,
    pub bluetooth_details: BluetoothDetails,
    pub sound_device: Option<DeviceInfo>,
    pub open_terminal: bool,
    pub cell_signal: bool,

    pub brightness_slider_state: Entity<SliderState>,
    pub brightness_slider_value: f32,

    pub volume_slider_state: Entity<SliderState>,
    pub volume_slider_value: f32,

    pub nm_tx: mpsc::Sender<NmEvents>,
    pub bt_tx: mpsc::Sender<BtEvents>,
    _subscriptions: Vec<Subscription>,
}

impl SettingsDrawer {
    pub fn new(
        cx: &mut Context<Self>,
        nm_tx: mpsc::Sender<NmEvents>,
        bt_tx: mpsc::Sender<BtEvents>,
    ) -> Self {
        let brightness_slider = cx.new(|_| SliderState::new());
        let b_subscription =
            cx.subscribe(&brightness_slider, |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.brightness_slider_value = *value;
                println!("brightness value: {:?}", this.brightness_slider_value);
                cx.notify();
            });

        let volume_slider = cx.new(|_| {
            SliderState::new()
                .default_value(20.)
                .pattern(widgets::SliderPattern::Bars)
        });
        let c_subscription = cx.subscribe(&volume_slider, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.volume_slider_value = *value;
            println!("volume value: {:?}", this.volume_slider_value);
            cx.notify();
        });

        let mut _subscriptions = vec![b_subscription, c_subscription];

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
            wireless_details: WirelessDetails {
                enabled: true,
                strength: 0,
                connected_network: None,
            },
            bluetooth_details: BluetoothDetails {
                enabled: false,
                devices: 0,
                connected_device: None,
            },
            sound_device: None,
            open_terminal: false,
            cell_signal: false,
            brightness_slider_state: brightness_slider,
            brightness_slider_value: 0.0,
            volume_slider_state: volume_slider,
            volume_slider_value: 0.0,
            nm_tx,
            bt_tx,
            _subscriptions,
        }
    }
}

impl Render for SettingsDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let wireless_icon = match self.wireless_details.enabled {
            true => match self.wireless_details.strength {
                0..=20 => IconName::WirelessLow,
                21..=50 => IconName::WirelessMedium,
                51..=75 => IconName::WirelessMedium,
                76..=100 => IconName::WirelessHigh,
                _ => IconName::WirelessOn,
            },
            false => IconName::WirelessOff,
        };
        let network_label = match self.wireless_details.enabled.clone() {
            true => self
                .wireless_details
                .connected_network
                .clone()
                .map(|s| s.ssid)
                .unwrap_or_else(|| "Wi-Fi".to_string()),
            false => "Wi-Fi".to_string(),
        };

        let bluetooth_icon = match self.bluetooth_details.enabled {
            true => match self.bluetooth_details.devices > 0 {
                true => IconName::BluetoothConnected,
                false => IconName::BluetoothOn,
            },
            false => IconName::BluetoothOff,
        };
        let bluetooth_label = match self.bluetooth_details.enabled {
            true => {
                if self.bluetooth_details.devices == 0 {
                    "Bluetooth".to_string()
                } else {
                    format!("{} Devices", self.bluetooth_details.devices)
                }
            }
            false => "Bluetooth".to_string(),
        };

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
                    .w(px(476.))
                    .bg(rgb(0x181818))
                    .p_6()
                    .gap_5()
                    .rounded(px(12.))
                    .child(
                        IconButton::new("id_rotation")
                            .icon(rotation_icon)
                            .active(self.rotation_on)
                            .active_icon_color(rgb(0x4892F1))
                            .active_bg_color(rgb(0x202020))
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
                            .active_icon_color(rgb(0x4892F1))
                            .active_bg_color(rgb(0x202020))
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
                            .active_bg_color(rgb(0x202020))
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
                            .active_bg_color(rgb(0x202020))
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
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("calc clicked");
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_camera")
                            .icon(IconName::Camera)
                            .icon_color(rgb(0xF4F4F4))
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("camera clicked");
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
                    .h(px(72.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .w_full()
                            .h_full()
                            .text_color(rgb(0xF4F4F4))
                            .text_lg()
                            .col_span(2)
                            .bg(rgb(0x202020))
                            .rounded(px(8.))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .items_center()
                                    .justify_around()
                                    .px_2()
                                    .child(
                                        IconButton::new("id_brightness")
                                            .icon(IconName::BrightnessHigh)
                                            .icon_color(rgb(0xF4F4F4))
                                            .size((px(36.), px(36.)))
                                            .bg_color(rgb(0x202020))
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
                                            .child(
                                                Slider::new(
                                                    "brightness-slider",
                                                    &self.brightness_slider_state,
                                                )
                                                .width(172.0)
                                                .height(56.0),
                                            ),
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
                            .rounded(px(8.))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .items_center()
                                    .justify_around()
                                    .pl_2()
                                    .child(
                                        IconButton::new("id_volume")
                                            .icon(IconName::VolumeMedium)
                                            .icon_color(rgb(0xF4F4F4))
                                            .size((px(36.), px(36.)))
                                            .bg_color(rgb(0x202020))
                                            .border(px(0.))
                                            .on_click(cx.listener(|_, _, _, _| {
                                                println!("volume clicked");
                                            })),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_center()
                                            .items_end()
                                            .w(px(172.0))
                                            .child(
                                                Slider::new(
                                                    "volume-slider",
                                                    &self.volume_slider_state,
                                                )
                                                .width(172.0),
                                            ),
                                    ),
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
                    .child(
                        IconButton::new("id_wireless")
                            .icon(wireless_icon)
                            .icon_color(rgb(0x4D4D4D)) // changes as per wireless state
                            .size((px(104.), px(104.)))
                            .active(self.wireless_details.enabled)
                            .active_icon_color(rgb(0x4892F1))
                            .active_bg_color(rgb(0x202020))
                            .label(network_label)
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    let mut nm_tx = this.nm_tx.clone();
                                    let is_enable = this.wireless_details.enabled;
                                    cx.background_executor()
                                        .spawn(async move {
                                            let _ = nm_tx
                                                .send(NmEvents::WirelessToggle {
                                                    enabled: !is_enable,
                                                })
                                                .await;
                                        })
                                        .detach();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_bluetooth")
                            .icon(bluetooth_icon)
                            .size((px(104.), px(104.)))
                            .label(bluetooth_label)
                            .active(self.bluetooth_details.enabled)
                            .active_icon_color(rgb(0x4892F1))
                            .active_bg_color(rgb(0x202020))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    let mut bt_tx = this.bt_tx.clone();
                                    let is_enable = this.bluetooth_details.enabled;
                                    cx.background_executor()
                                        .spawn(async move {
                                            let _ = bt_tx
                                                .send(BtEvents::BluetoothToggle {
                                                    enabled: !is_enable,
                                                })
                                                .await;
                                        })
                                        .detach();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_terminal")
                            .icon(IconName::Terminal)
                            .size((px(104.), px(104.)))
                            .label("Terminal")
                            .icon_color(rgb(0xF4F4F4))
                            .active_icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("terminal clicked");
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
                            .active_bg_color(rgb(0x202020))
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
