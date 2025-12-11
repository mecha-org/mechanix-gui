use chrono::Local;
pub mod icon;  
mod widgets;
mod modals;
use crate::events::{BrightnessEvents, NmEvents, VolumeEvents};
use crate::get_wireless_strength_icon;
use crate::prelude::*;
use crate::services::DEFAULT_MIN_BRIGHTNESS;
use crate::ui::icon::Icon;
use crate::ui::modals::{BluetoothWindow, DisplayWindow, ExtendScreenOptions, PerformanceWindow, SoundWindow, WirelessWindow};
use crate::{
    events::BtEvents,
    ui::{
        icon::IconName,
        widgets::{IconButton, Slider, SliderEvent, SliderState},
    },
};
use bluez::interfaces::device::BluetoothDevice;
use futures::{SinkExt, channel::mpsc};
use gpui::*;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;
use upower::interfaces::device::BatteryState;

const NAVBAR_SIZE: (f32, f32) = (180., 29.);
const APP_SIZE: (f32, f32) = (540., 620.);
const MODAL_SIZE: (f32, f32) = (478., 392.); 

pub enum PowerMode {
    High,
    Balanced,
    Low,
}

pub struct WirelessDetails {
    pub enabled: bool,      
    pub connected_network: Option<WirelessNetworkInfo>,
    pub networks: Option<Vec<WirelessNetworkInfo>>,
}

pub struct BluetoothDetails {
    pub enabled: bool,
    pub devices: u8,
    pub connected_device: Option<String>,
    pub available_devices: Option<Vec<BluetoothDevice>>,
}

pub fn get_current_datetime() -> String {
    let now = Local::now();
    format!("{}", now.format("%d %b, %H:%M"))
}

pub struct SettingsDrawer {
    pub current_time_date: String,
    pub settings_active: bool,

    pub battery_state: BatteryState,
    pub battery_percent: u8,

    pub open_power_options: bool,

    pub rotation_on: bool,
    pub airplane_mode: bool,
    pub screen_mirroring: bool,
    pub power_mode: PowerMode,
    pub microphone_recording: bool,
    pub screen_recording: bool,

    pub wireless_details: WirelessDetails,
    pub bluetooth_details: BluetoothDetails,
    pub open_terminal: bool,
    pub cell_signal: bool,

    pub brightness_slider_state: Entity<SliderState>,
    pub brightness_slider_value: f32,
    pub auto_brightness: bool,
    pub dark_mode: bool,

    pub volume_slider_state: Entity<SliderState>,
    pub volume_slider_value: f32,
    pub volume_device_name: Option<String>,
    pub volume_mute: bool,

    pub show_wireless_modal : bool,
    pub show_bluetooth_modal : bool,

    pub nm_tx: mpsc::Sender<NmEvents>,
    pub bt_tx: mpsc::Sender<BtEvents>,
    pub volume_tx: mpsc::Sender<VolumeEvents>,
    _subscriptions: Vec<Subscription>,

    position: f32,
    drag_offset: Option<f32>,
    drag_start_pos: f32,
}

impl SettingsDrawer {
    pub fn new(
        cx: &mut Context<Self>,
        nm_tx: mpsc::Sender<NmEvents>,
        bt_tx: mpsc::Sender<BtEvents>,
        volume_tx: mpsc::Sender<VolumeEvents>,
        brightness_tx: mpsc::Sender<BrightnessEvents>,
    ) -> Self {
        let volume_tx_for_slider = volume_tx.clone();
        let brightness_slider = cx.new(|_| SliderState::new());
        let b_subscription = cx.subscribe(
            &brightness_slider,
            move |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.brightness_slider_value = *value;

                let mut brightness_tx = brightness_tx.clone();
                let brightness_value = *value;
                cx.background_executor()
                    .spawn(async move {
                        let _ = brightness_tx
                            .send(BrightnessEvents::BrightnessChanged {
                                value: brightness_value,
                            })
                            .await;
                    })
                    .detach();

                    // Update the slider state
                      let value = if *value <= DEFAULT_MIN_BRIGHTNESS { DEFAULT_MIN_BRIGHTNESS } else { *value };
                    this.brightness_slider_state.update(cx, |state, _cx| {
                        state.value = value.clamp(state.min, state.max);
                    });

                cx.notify();
            },
        );

        let volume_slider = cx.new(|_| {
            SliderState::new()
                .default_value(0.)
                .pattern(widgets::SliderPattern::Bars)
        });

        let c_subscription =
            cx.subscribe(&volume_slider, move |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.volume_slider_value = *value;

                let sink_name = this.volume_device_name.clone().unwrap_or_else(|| "default".to_string());
                let volume = *value;
                let mut volume_tx_1 = volume_tx_for_slider.clone();

                let _ = cx
                    .background_executor()
                    .spawn(async move {
                        let _ = volume_tx_1
                            .send(VolumeEvents::VolumeChanged {
                                name: sink_name,
                                value: volume,
                            })
                            .await;
                    })
                    .detach();

                    // Update the slider state
                    this.volume_mute = *value <= 0.0;
                    this.volume_slider_value = if this.volume_mute { 0.0 } else { *value };
                    this.volume_slider_state.update(cx, |state, _cx| {
                        state.value = value.clamp(state.min, state.max);
                    });

                cx.notify();
            });

        let mut _subscriptions = vec![b_subscription, c_subscription];

        Self {
            current_time_date: get_current_datetime(),
            settings_active: false,
            battery_state: BatteryState::Unknown,
            battery_percent: 0,
            open_power_options: false,
            rotation_on: false,
            airplane_mode: false,
            screen_mirroring: false,
            power_mode: PowerMode::Balanced,
            microphone_recording: false,
            screen_recording: false,
            wireless_details: WirelessDetails {
                enabled: true,
                connected_network: None,
                networks: None,
            },
            bluetooth_details: BluetoothDetails {
                enabled: false,
                devices: 0,
                connected_device: None,
                available_devices: None,
            },
            volume_device_name: None,
            open_terminal: false,
            cell_signal: false,
            brightness_slider_state: brightness_slider,
            brightness_slider_value: 0.0,
            auto_brightness: false,
            dark_mode: false,

            volume_slider_state: volume_slider,
            volume_slider_value: 0.0,
            volume_mute: false,
            nm_tx,
            bt_tx,
            volume_tx,
            _subscriptions,

            show_wireless_modal : false,
            show_bluetooth_modal : false,

            position: Self::closed_pos(),
            drag_offset: None,
            drag_start_pos: 0.0,
        }
    }
}

impl Render for SettingsDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_y = 0.;
        let closed_y = Self::closed_pos();

        let threshold_px = 40.;

        div()
            .w_full()
            .h_full()
            .on_mouse_move(
                cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                    if let Some(offset) = this.drag_offset {
                        let new_y = event.position.y.to_f64() as f32 - offset;
                        this.position = new_y.clamp(open_y, closed_y);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.drag_offset.is_some() {
                        this.drag_offset = None;

                        let target;
                        let started_closed = this.drag_start_pos > (closed_y / 2.0);

                        if started_closed {
                            if this.position < (closed_y - threshold_px) {
                                target = open_y;
                                this.update_input_regions(window, false);
                            } else {
                                target = closed_y;
                                this.update_input_regions(window, true);
                            }
                        } else {
                            if this.position > (open_y + threshold_px) {
                                target = closed_y;
                                this.update_input_regions(window, true);
                            } else {
                                target = open_y;
                                this.update_input_regions(window, false);
                            }
                        }
                        this.snap_to(target, cx);
                        cx.notify();
                    }
                }),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .absolute()
                    .top(px(self.position))
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .h(px(NAVBAR_SIZE.1))
                            .child(
                                img(IconName::Navbar.resolve())
                                    .id("settings-drawer-navbar")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                            cx.stop_propagation();
                                            this.drag_start_pos = this.position;
                                            this.drag_offset = Some(
                                                event.position.y.to_f64() as f32 - this.position,
                                            );
                                            cx.notify();
                                        }),
                                    ),
                            ),
                    )
                    .child(self.drawer_items(window, cx)),
            )
    }
}

impl SettingsDrawer {
    fn closed_pos() -> f32 {
        APP_SIZE.1 - NAVBAR_SIZE.1
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<SettingsDrawer>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    // Check if animation is done
                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.position = target;
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position = current;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }
    fn update_input_regions(&self, window: &mut Window, open: bool) {
        let mut regions = Vec::new();

        if open {
            regions.push(Bounds {
                origin: point(px(APP_SIZE.0 - NAVBAR_SIZE.0), px(Self::closed_pos())),
                size: size(px(NAVBAR_SIZE.0), px(NAVBAR_SIZE.1)),
            });
        } else {
            regions.push(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(APP_SIZE.0), px(APP_SIZE.1)),
            });
        }
        window.set_input_regions(Some(regions));
    }

    fn drawer_items(
        &mut self,
        window: &mut Window,
        cx: &mut Context<SettingsDrawer>,
    ) -> impl IntoElement {

        let window_bounds = window.bounds();

        let volume_icon = if self.volume_mute {
            IconName::VolumeOff
        } else {
            if self.volume_slider_value >= 0.0 && self.volume_slider_value <= 33.0 {
                IconName::VolumeLow
            } else if self.volume_slider_value > 33.0 && self.volume_slider_value <= 66.0 {
                IconName::VolumeMedium
            } else {
                IconName::VolumeHigh
            }
        };
        let volume_icon_color = if self.volume_mute {
            MUTE_SOUND_COLOR
        } else {
            UNMUTE_SOUND_COLOR
        };

        let brightness_icon =
            if self.brightness_slider_value >= 0.0 && self.brightness_slider_value <= 33.0 {
                IconName::BrightnessLow
            } else if self.brightness_slider_value > 33.0 && self.brightness_slider_value <= 66.0 {
                IconName::BrightnessMedium
            } else {
                IconName::BrightnessHigh
            };

        let mut wireless_icon = IconName::WifiOff;
        let mut network_label = "Wi-Fi".to_string();
        let wireless_connected_network =  self
                .wireless_details
                .connected_network
                .clone();
        let wireless_enable = self.wireless_details.enabled;
        if wireless_enable && wireless_connected_network.is_some() {
            network_label = wireless_connected_network.clone()
                .map(|s| s.ssid)
                .unwrap_or_else(|| "Wi-Fi".to_string());

            wireless_icon = if network_label == "Wi-Fi" {
                IconName::ConnectedWifiOn
            } else {
                let signal_strength = wireless_connected_network.clone().map(|info| info.signal_strength).unwrap_or_else(|| 0);
                get_wireless_strength_icon(wireless_enable, signal_strength, "Open".to_string())  // intentionally open as no lock to show in view
            };
        }

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

        // Add extened screen icons when extended screen is detected
        let screen_mirroring_icon = if self.screen_mirroring {
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
            PowerMode::High => rgb(AMBER_600),     // blue
            PowerMode::Balanced => rgb(DARK_NEUTRAL_100), // gray
            PowerMode::Low => rgb(AMBER_600),      // yellow
        };

        div()
            .id("id_drawer")
            .flex()
            .flex_col()
            .bg(rgb(DARK_NEUTRAL_1000))
            .w_full()
            .h_full()
            .content_stretch()
            .px_8()
            .child(
                // status row
                div()
                    .w_full()
                    .flex()
                    .h(px(32.82))
                    .mt_7()
                    .py_1()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .child(self.current_time_date.clone())
                            .text_xl()
                            .text_color(rgb(TEXT_COLOR)),
                    )
                    .child(
                        div()
                        .id("id_power")
                        .child(
                            Icon::new(IconName::Power)
                                .text_color(rgb(0xF4F4F4))
                                .size((px(24.), px(24.)))
                        )
                        // .on_click(cx.listener(
                        //     move |_,
                        //             _event: &ClickEvent,
                        //             _window: &mut Window,
                        //             cx: &mut Context<Self>| {
                        //         println!("power clicked");

                        //         let popup_origin =
                        //             point(window_bounds.origin.x, window_bounds.origin.y);

                        //         let popup_bounds = Bounds {
                        //             origin: popup_origin,
                        //             size: size(px(476.0), px(180.0)),
                        //         };

                        //         cx.open_window(
                        //             WindowOptions {
                        //                 titlebar: None,
                        //                 kind: WindowKind::PopUp,
                        //                 is_movable: false,
                        //                 window_bounds: Some(WindowBounds::Windowed(
                        //                     popup_bounds,
                        //                 )),
                        //                 ..Default::default()
                        //             },
                        //             |_, cx| {
                        //                 cx.new(|_| BatteryWindow::new("Battery".to_string()))
                        //             },
                        //         )
                        //         .unwrap();
                        //     },
                        // )),
                    ),
            )
            .child(
                div()
                    .grid()
                    .grid_rows(2)
                    .grid_cols(4)
                    .pt_4()
                    .gap_4()
                    .content_center()
                    .rounded(px(12.))
                    .child(
                        IconButton::new("id_rotation")
                        .size((px(104.), px(88.)))
                            .icon(rotation_icon)
                            .active(self.rotation_on)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
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
                            .size((px(104.), px(88.)))
                            .icon_color(rgb(DARK_NEUTRAL_100))
                            .active(self.airplane_mode)
                            .active_icon_color(rgb(DARK_NEUTRAL_0))
                            .active_bg_color(rgb(AMBER_600))
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
                            .size((px(104.), px(88.)))
                            .active(self.screen_mirroring)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            // .on_click(cx.listener(  // toggle
                            //     |this: &mut SettingsDrawer,
                            //      _event: &ClickEvent,
                            //      _window: &mut Window,
                            //      cx: &mut Context<Self>| {
                            //         this.screen_mirroring = !this.screen_mirroring;
                            //         cx.notify();
                            //     },
                            // )),
                            .on_click(cx.listener(  // long press
                            move |_,
                                _event: &ClickEvent,
                                _window: &mut Window,
                                cx: &mut Context<Self>| {
                                
                                let popup_bounds = Bounds::centered(None, size(px(MODAL_SIZE.0), px(MODAL_SIZE.1)), cx);    
                                
                                cx.open_window(
                                WindowOptions {
                                    titlebar: None,
                                    kind: WindowKind::PopUp,
                                    is_movable: false,
                                    window_bounds:Some(
                                            WindowBounds::Windowed(
                                                popup_bounds,
                                            ),
                                        ),
                                    ..Default::default()
                                },
                                |_, cx| {
                                    cx.new(|_| 
                                        ExtendScreenOptions::new("Extended Screen".to_string())
                                    )
                                },
                                ).unwrap();

                            },
                        ))
                    )
                    .child(
                           IconButton::new("id_terminal")
                            .icon(IconName::Terminal)
                            .size((px(104.), px(88.)))
                            .icon_color(rgb(DARK_NEUTRAL_100))
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            .on_click(cx.listener(
                                |_,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("terminal clicked");
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_microphone")
                            .icon(IconName::MicroPhoneOff)
                            .size((px(104.), px(88.)))
                            .active(self.microphone_recording)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.microphone_recording = !this.microphone_recording;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_screen_recording")
                            .icon(IconName::ScreenRecordingOff)
                            .size((px(104.), px(88.)))
                            .active(self.screen_recording)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
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
                        IconButton::new("id_settings")
                            .icon(IconName::Settings)
                            .size((px(104.), px(88.)))
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            .on_click(cx.listener(
                                |_,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("settigns clicked");
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_camera")
                            .icon(IconName::CameraOff) 
                            .size((px(104.), px(88.)))
                            // .icon_color(rgb(AMBER_600)) // on press change ICON CameraON
                            .icon_color(rgb(DARK_NEUTRAL_100))
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            .on_click(cx.listener(
                                |_,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("camera clicked");
                                    cx.notify();
                                },
                            )),
                    )
                  
            ) 
            .child(
                div()
                    .grid()
                    .grid_rows(2)
                    .grid_cols(4)
                    .pt_4()
                    .gap_4()
                    .content_center()
                    .rounded(px(12.))
                    .child(
                        IconButton::new("id_wireless")
                            .icon(
                                 Icon::new(wireless_icon)
                                    .size((px(36.), px(36.)))
                            )
                            .size((px(104.), px(104.)))
                            .active(self.wireless_details.enabled)
                            .label(network_label)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10)) // 10% - ok - keeping while it is active
                            // .on_click(cx.listener(
                            //     |this: &mut SettingsDrawer,
                            //      _event: &ClickEvent,
                            //      _window: &mut Window,
                            //      cx: &mut Context<Self>| {
                            //         let mut nm_tx = this.nm_tx.clone();
                            //         let is_enable = this.wireless_details.enabled;
                            //         cx.background_executor()
                            //             .spawn(async move {
                            //                 let _ = nm_tx
                            //                     .send(NmEvents::WirelessToggle {
                            //                         enabled: !is_enable,
                            //                     })
                            //                     .await;
                            //             })
                            //             .detach();
                            //     },
                            // )),
                            .on_click(cx.listener(  // TEMP; TODO: long press open modal
                                move |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("wireless clicked");

                                        let popup_bounds = Bounds::centered(None, size(px(MODAL_SIZE.0), px(MODAL_SIZE.1)), cx);    
                                        cx.open_window(
                                        WindowOptions {
                                            
                                            titlebar: None,
                                            kind: WindowKind::PopUp,
                                            is_movable: false,
                                            window_bounds:Some(
                                                    WindowBounds::Windowed(
                                                        popup_bounds,
                                                    ),
                                                ),
                                            ..Default::default()
                                        },
                                        |_, cx| {
                                            cx.new(|_| 
                                                WirelessWindow::new("Wireless".to_string(), this.wireless_details.networks.clone().unwrap(), this.nm_tx.clone()) 
                                        )
                                        },
                                    )
                                    .unwrap();

                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_bluetooth")
                            .icon(bluetooth_icon)
                            .size((px(104.), px(104.)))
                            .label(bluetooth_label)
                            .active(self.bluetooth_details.enabled)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            // .on_click(cx.listener(
                            //     |this: &mut SettingsDrawer,
                            //      _event: &ClickEvent,
                            //      _window: &mut Window,
                            //      cx: &mut Context<Self>| {
                            //         let mut bt_tx = this.bt_tx.clone();
                            //         let is_enable = this.bluetooth_details.enabled;
                            //         cx.background_executor()
                            //             .spawn(async move {
                            //                 let _ = bt_tx
                            //                     .send(BtEvents::BluetoothToggle {
                            //                         enabled: !is_enable,
                            //                     })
                            //                     .await;
                            //             })
                            //             .detach();
                            //     },
                            // )),
                             .on_click(cx.listener(  // TEMP; TODO: long press open modal
                                move |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    println!("bluetooth clicked");

                                     let popup_origin = point(
                                            window_bounds.origin.x,
                                            window_bounds.origin.y,
                                        );

                                        let popup_bounds = Bounds {
                                            origin: popup_origin,
                                            size: size(px(476.0), px(338.0)),
                                        };
                                     
                                        cx.open_window(
                                        WindowOptions {
                                            titlebar: None,
                                            kind: WindowKind::PopUp,
                                            is_movable: false,
                                            window_bounds:Some(
                                                    WindowBounds::Windowed(
                                                        popup_bounds,
                                                    ),
                                                ),
                                            ..Default::default()
                                        },
                                        |_, cx| {
                                            cx.new(|_| 
                                                BluetoothWindow::new("Bluetooth".to_string(), this.bluetooth_details.available_devices.clone(), this.bt_tx.clone()) 
                                        )
                                        },
                                    )
                                    .unwrap();

                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_power_mode")
                            .icon(power_mode_icon)
                            .size((px(104.), px(104.)))
                            .label(format!("{}% ", self.battery_percent))
                            .icon_color(power_mode_icon_color)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
                            .on_click(cx.listener(  
                                move |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    let popup_bounds = Bounds::centered(None, size(px(MODAL_SIZE.0), px(MODAL_SIZE.1)), cx);    
                                    cx.open_window(
                                    WindowOptions {
                                        titlebar: None,
                                        kind: WindowKind::PopUp,
                                        is_movable: false,
                                        window_bounds:Some(
                                                WindowBounds::Windowed(
                                                    popup_bounds,
                                                ),
                                            ),
                                        ..Default::default()
                                    },
                                    |_, cx| {
                                        cx.new(|_| 
                                            PerformanceWindow::new("Battery".to_string())
                                            
                                    )}).unwrap();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("id_cell_signal")
                            .icon(IconName::CellSignalNone)
                            .size((px(104.), px(104.)))
                            .label("No SIM")
                            .active(self.cell_signal)
                            .active_icon_color(rgb(AMBER_600))  
                            .active_bg_color(rgba(AMBER_600_10))
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
                    )
                    .child(
                          div()
                            .id("id_display")
                            .flex()
                            .items_center()
                            .w_full()
                            .h_full()
                            .text_lg()
                            .col_span(2)
                            .bg(rgb(DARK_NEUTRAL_900))
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
                                            .icon(brightness_icon)
                                            .icon_color(rgb(BRIGHTNESS_ICON_COLOR))
                                            .size((px(32.), px(32.)))
                                            .bg_color(rgb(DARK_NEUTRAL_900))
                                            .active_bg_color(rgb(DARK_NEUTRAL_900))
                                            .border(px(0.))
                                           .on_click(cx.listener(  // todo: show modal on long press
                                                move |this: &mut SettingsDrawer,
                                                _event: &ClickEvent,
                                                _window: &mut Window,
                                                cx: &mut Context<Self>| {
                                                    let popup_bounds = Bounds::centered(None, size(px(MODAL_SIZE.0), px(MODAL_SIZE.1)), cx);    
                                                    cx.open_window(
                                                    WindowOptions {
                                                        titlebar: None,
                                                        kind: WindowKind::PopUp,
                                                        is_movable: false,
                                                        window_bounds:Some(
                                                                WindowBounds::Windowed(
                                                                    popup_bounds,
                                                                ),
                                                            ),
                                                        ..Default::default()
                                                            },
                                                            |_, cx| {
                                                                cx.new(|_| 
                                                                    DisplayWindow::new("Display brightness".to_string(), this.auto_brightness, this.dark_mode)
                                                                )
                                                            }).unwrap();
                                                        },
                                    ))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_center()
                                            .items_center()
                                            .w(px(167.0))
                                            .child(
                                                Slider::new(
                                                    "brightness-slider",
                                                    &self.brightness_slider_state,
                                                )
                                                .height(66.0),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("id_sound")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_full()
                            .h_full()
                            .text_lg()
                            .col_span(2)
                            .bg(rgb(DARK_NEUTRAL_900))
                            .rounded(px(8.))
                            .on_click(cx.listener(  
                                move |_,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    let popup_bounds = Bounds::centered(None, size(px(476.0), px(400.0)), cx);
                                     
                                    cx.open_window(
                                    WindowOptions {
                                        titlebar: None,
                                        kind: WindowKind::PopUp,
                                        is_movable: false,
                                        window_bounds:Some(
                                                WindowBounds::Windowed(
                                                    popup_bounds,
                                                ),
                                            ),
                                        ..Default::default()
                                    },
                                    |_, cx| {
                                        cx.new(|_| 
                                            SoundWindow::new("Sound".to_string())
                                        )
                                    },
                                    ).unwrap();

                                },
                            ))
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
                                            .icon(volume_icon)
                                            .icon_color(rgb(volume_icon_color))
                                            .size((px(32.), px(32.)))
                                            .bg_color(rgb(DARK_NEUTRAL_900))
                                            .active_bg_color(rgb(DARK_NEUTRAL_900))
                                            .border(px(0.))
                                            .on_click(cx.listener(
                                        |this: &mut SettingsDrawer,
                                        _event: &ClickEvent,
                                        _window: &mut Window,
                                        cx: &mut Context<Self>| {
                                            let mut volume_tx = this.volume_tx.clone();
                                            this.volume_mute = !this.volume_mute;
                                            let is_mute = this.volume_mute;
                                            let sink_name = this.volume_device_name.clone().unwrap_or_else(|| "default".to_string());

                                            if is_mute {
                                                cx.background_executor()
                                                .spawn(async move {
                                                    let _ = volume_tx
                                                        .send(VolumeEvents::MuteSink { name: sink_name.clone() })
                                                        .await;
                                                })
                                                .detach();
                                            } else {
                                                cx.background_executor()
                                                .spawn(async move {
                                                    let _ = volume_tx
                                                        .send(VolumeEvents::UnmuteSink { name: sink_name.clone() })
                                                        .await;
                                                })
                                                .detach();
                                            }
                                        
                                        },
                                    )),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_center()
                                            .items_end()
                                            .w(px(167.0))
                                            .child(
                                                Slider::new(
                                                    "volume-slider",
                                                    &self.volume_slider_state,
                                                )
                                                .height(66.0),
                                            ),
                                    ),
                            ),
                    ),
            )

    }
}
