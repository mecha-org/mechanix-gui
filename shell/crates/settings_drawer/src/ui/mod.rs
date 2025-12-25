pub mod icon;
mod modals;
mod widgets;
use gpui::prelude::FluentBuilder;
use shell_state::DEFAULT_MIN_BRIGHTNESS;
use shell_state::{BrightnessMessage, BtMessage, NmMessage, ShellState, VolumeMessage};

use crate::constants::*;
use crate::helper::get_wireless_strength_icon;
use crate::ui::icon::Icon;
use crate::ui::modals::*;
use crate::ui::{
    icon::IconName,
    widgets::{IconButton, Slider, SliderEvent, SliderState},
};
use bluez::interfaces::device::BluetoothDevice;
use futures::{SinkExt, channel::mpsc};
use gpui::*;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;

const NAVBAR_SIZE: (f32, f32) = (198.5, 28.29);
const APP_SIZE: (f32, f32) = (540., 620.);

const MIN_MODAL_SIZE_1: (f32, f32) = (133., 110.);
const MIN_MODAL_SIZE_2: (f32, f32) = (203., 168.);
const FINAL_MODAL_SIZE: (f32, f32) = (478., 392.);

const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 4;
const ICON_W: f32 = 104.0;
const ICON_H: f32 = 104.0;

#[derive(PartialEq)]
pub enum ModalAnimationState {
    Opening,
    Closing,
    None,
}

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
    pub connected_devices: u8,
    pub connected_device: Option<String>,
    pub available_devices: Option<Vec<BluetoothDevice>>,
}

pub struct SettingsDrawer {
    pub current_time_date: String,
    pub open_power_options: bool,

    pub rotation_on: bool,
    pub airplane_mode: bool,
    pub screen_mirroring: bool,
    pub open_terminal: bool,

    pub microphone_recording: bool,
    pub screen_recording: bool,
    pub settings_active: bool,
    // camera
    pub wireless_details: WirelessDetails,
    pub bluetooth_details: BluetoothDetails,
    pub power_mode: PowerMode,
    pub battery_percent: u8,
    pub cell_signal: bool,

    pub brightness_slider_state: Entity<SliderState>,
    pub brightness_slider_value: f32,
    pub auto_brightness: bool,
    pub dark_mode: bool,

    pub volume_slider_state: Entity<SliderState>,
    pub volume_slider_value: f32,
    pub volume_device_name: Option<String>,
    pub volume_mute: bool,

    pub nm_tx: Option<mpsc::Sender<NmMessage>>,
    pub bt_tx: Option<mpsc::Sender<BtMessage>>,
    pub volume_tx: Option<mpsc::Sender<VolumeMessage>>,
    pub brightness_tx: Option<mpsc::Sender<BrightnessMessage>>,

    pub open_modal: bool,
    pub animation_progress: f32,
    pub animation_state: ModalAnimationState,
    pub modal_size: (f32, f32),

    pub modal_start_center: (f32, f32),
    pub modal_origin_center: (f32, f32),
    pub modal_current_center: (f32, f32),
    pub modal_target_center: (f32, f32),

    pub current_modal: ModalKind,
    pub wireless_modal_scroll: WirelessModalScroll,
    pub bluetooth_modal_scroll: BluetoothModalScroll,

    // _subscriptions: Vec<Subscription>,
    position: f32,
    drag_offset: Option<f32>,
    drag_start_pos: f32,
}

#[derive(Clone, Debug)]
pub enum ModalKind {
    None,
    WirelessModal,
    BluetoothModal,
    ScreenMirroring,
    PerformanceModal,
    DisplayModal,
    SoundModal,
}

impl SettingsDrawer {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let brightness_slider = cx.new(|_| SliderState::new());
        let volume_slider = cx.new(|_| {
            SliderState::new()
                .default_value(0.)
                .pattern(widgets::SliderPattern::Bars)
        });

        let b_subscription = cx.subscribe(
            &brightness_slider,
            move |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.brightness_slider_value = *value;

                let mut brightness_tx = this.brightness_tx.clone().unwrap();
                let brightness_value = *value;
                cx.background_executor()
                    .spawn(async move {
                        let _ = brightness_tx
                            .send(BrightnessMessage::BrightnessChanged {
                                value: brightness_value,
                            })
                            .await;
                    })
                    .detach();

                // Update the slider state
                let value = if *value <= DEFAULT_MIN_BRIGHTNESS {
                    DEFAULT_MIN_BRIGHTNESS
                } else {
                    *value
                };
                this.brightness_slider_state.update(cx, |state, _cx| {
                    state.value = value.clamp(state.min, state.max);
                });

                cx.notify();
            },
        );

        let c_subscription =
            cx.subscribe(&volume_slider, move |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.volume_slider_value = *value;

                let sink_name = this
                    .volume_device_name
                    .clone()
                    .unwrap_or_else(|| "default".to_string());
                let volume = *value;
                let mut volume_tx_1 = this.volume_tx.clone().unwrap();

                let _ = cx
                    .background_executor()
                    .spawn(async move {
                        let _ = volume_tx_1
                            .send(VolumeMessage::VolumeChanged {
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
            current_time_date: "".to_string(),
            settings_active: false,
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
                connected_devices: 0,
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
            nm_tx: None,
            bt_tx: None,
            volume_tx: None,
            brightness_tx: None,
            open_modal: false,
            animation_progress: 0.0,
            animation_state: ModalAnimationState::None,
            modal_size: (MIN_MODAL_SIZE_1.0, MIN_MODAL_SIZE_1.1),
            modal_start_center: (0.0, 0.0),
            modal_origin_center: (0.0, 0.0),
            modal_current_center: (0.0, 0.0),
            modal_target_center: (0.0, 0.0),

            // _subscriptions,
            current_modal: ModalKind::None,
            wireless_modal_scroll: WirelessModalScroll::new(),
            bluetooth_modal_scroll: BluetoothModalScroll::new(),
            position: Self::closed_pos(),
            drag_offset: None,
            drag_start_pos: 0.0,
        }
    }

    fn start_animation(&mut self, event: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        let mouse = event.mouse_position().unwrap();

        // Start from clicked icon center
        self.modal_start_center = Self::clicked_item_center(mouse.x.into(), mouse.y.into());
        self.modal_origin_center = self.modal_start_center;

        self.modal_current_center = self.modal_start_center;

        // End at APP center
        self.modal_target_center = (APP_SIZE.0 / 2.0, APP_SIZE.1 / 2.0);

        self.modal_size = MIN_MODAL_SIZE_1;
        self.animation_progress = 0.0;
        self.animation_state = ModalAnimationState::Opening;
        self.open_modal = true;

        cx.notify();
    }

    fn start_close_animation(&mut self, cx: &mut Context<Self>) {
        self.animation_progress = 1.0;
        self.animation_state = ModalAnimationState::Closing;
        cx.notify();
    }

    fn clicked_item_center(mouse_x: f32, mouse_y: f32) -> (f32, f32) {
        let grid_w = GRID_COLS as f32 * ICON_W;
        let grid_h = GRID_ROWS as f32 * ICON_H;

        let origin_x = (APP_SIZE.0 - grid_w) / 2.0;
        let origin_y = (APP_SIZE.1 - grid_h) / 2.0;

        let local_x = (mouse_x - origin_x).clamp(0.0, grid_w - 1.0);
        let local_y = (mouse_y - origin_y).clamp(0.0, grid_h - 1.0);

        let col = (local_x / ICON_W).floor() as usize;
        let row = (local_y / ICON_H).floor() as usize;

        (
            origin_x + col as f32 * ICON_W + ICON_W / 2.0,
            origin_y + row as f32 * ICON_H + ICON_H / 2.0,
        )
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
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if let Some(offset) = this.drag_offset {
                    let new_y = event.position.y.to_f64() as f32 - offset;
                    this.position = new_y.clamp(open_y, closed_y);
                    cx.notify();
                }
            }))
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

        if matches!(
            self.animation_state,
            ModalAnimationState::Opening | ModalAnimationState::Closing
        ) {
            match self.animation_state {
                ModalAnimationState::Opening => {
                    self.animation_progress += 0.08;
                }
                ModalAnimationState::Closing => {
                    self.animation_progress -= 0.08;
                }
                _ => {}
            }

            let t = self.animation_progress.clamp(0.0, 1.0);

            let ease = 1.0 - (1.0 - t).powi(3);

            // Center interpolation
            self.modal_current_center = (
                self.modal_origin_center.0
                    + (self.modal_target_center.0 - self.modal_origin_center.0) * ease,
                self.modal_origin_center.1
                    + (self.modal_target_center.1 - self.modal_origin_center.1) * ease,
            );

            // Size interpolation
            self.modal_size = (
                MIN_MODAL_SIZE_1.0 + (FINAL_MODAL_SIZE.0 - MIN_MODAL_SIZE_1.0) * ease,
                MIN_MODAL_SIZE_1.1 + (FINAL_MODAL_SIZE.1 - MIN_MODAL_SIZE_1.1) * ease,
            );

            // End conditions
            if self.animation_state == ModalAnimationState::Opening && t >= 1.0 {
                self.animation_state = ModalAnimationState::None;
                self.modal_current_center = self.modal_target_center;
                self.modal_size = FINAL_MODAL_SIZE;
            } else if self.animation_state == ModalAnimationState::Closing && t <= 0.0 {
                self.animation_state = ModalAnimationState::None;
                self.open_modal = false;
                self.modal_current_center = self.modal_origin_center;
                self.modal_size = MIN_MODAL_SIZE_1;
            } else {
                window.request_animation_frame();
            }
        }

        div()
            .id("root")
            .relative()
            .w(px(APP_SIZE.0))
            .h(px(APP_SIZE.1))
            .child(
                div()
                    .id("main_container")
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .px_8()
                    .bg(rgb(DARK_NEUTRAL_1000))
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
                            .child(self.render_power_button(cx)),
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
                            .child(self.render_rotation(cx))
                            .child(self.render_airplane_mode(cx))
                            .child(self.render_screen_mirroring(cx))
                            .child(self.render_terminal(cx))
                            .child(self.render_microphone_recording(cx))
                            .child(self.render_screen_recording(cx))
                            .child(self.render_settings(cx))
                            .child(self.render_camera(cx)),
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
                            .child(self.render_wireless(cx))
                            .child(self.render_bluetooth(cx))
                            .child(self.render_battery_performance(cx))
                            .child(self.render_cell_signal(cx))
                            .child(self.render_brightness_control_div(cx))
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
                                    .child(self.render_volume_control(cx)),
                            ),
                    )
                    .when(self.open_modal, |content_div| {
                        content_div
                            .relative()
                            .child(
                                div()
                                    .id("modal-bg")
                                    .absolute()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .top(px(0.))
                                    .left(px(0.))
                                    .w_full()
                                    .h_full()
                                    .bg(rgb(0x000000))
                                    .opacity(0.6)
                                    .on_click(cx.listener(
                                        |this: &mut SettingsDrawer, _, widnow, cx| {
                                            Self::start_close_animation(this, cx);
                                        },
                                    ))
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .on_mouse_up(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    }),
                            )
                            .child(
                                div()
                                    .id("modal-container")
                                    .absolute()
                                    .left(px(0.))
                                    .top(px(0.))
                                    .size_full()
                                    .child(
                                        div()
                                            .id("modal-item")
                                            .absolute()
                                            .flex()
                                            .left(px(self.modal_current_center.0
                                                - self.modal_size.0 / 2.0))
                                            .top(px(self.modal_current_center.1
                                                - self.modal_size.1 / 2.0))
                                            .w(px(self.modal_size.0))
                                            .h(px(self.modal_size.1))
                                            .bg(if self.modal_size.0 == MIN_MODAL_SIZE_1.0 {
                                                rgba(AMBER_600_10)
                                            } else {
                                                rgb(DARK_NEUTRAL_900)
                                            })
                                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                                px(16.)
                                            } else {
                                                px(18.)
                                            })
                                            .rounded_xl()
                                            .border_1()
                                            .border_color(rgb(AMBER_800))
                                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                                cx.stop_propagation()
                                            })
                                            .on_mouse_up(MouseButton::Left, |_, _, cx| {
                                                cx.stop_propagation()
                                            })
                                            .on_click(|_, _window, cx| cx.stop_propagation())
                                            .when(self.modal_size > MIN_MODAL_SIZE_2, |this| {
                                                this.child(match self.current_modal {
                                                    ModalKind::ScreenMirroring => {
                                                        self.render_extended_screen_options(cx)
                                                    }
                                                    ModalKind::WirelessModal => {
                                                        self.render_wireless_modal(cx)
                                                    }
                                                    ModalKind::BluetoothModal => {
                                                        self.render_bluetooth_modal(cx)
                                                    }
                                                    ModalKind::SoundModal => {
                                                        self.render_sound_modal(cx)
                                                    }
                                                    ModalKind::PerformanceModal => {
                                                        self.render_battery_performance_modal(cx)
                                                    }
                                                    ModalKind::DisplayModal => {
                                                        self.render_display_modal(cx)
                                                    }
                                                    ModalKind::None => Empty.into_any(),
                                                })
                                            }),
                                    ),
                            )
                    }),
            )
    }

    fn render_power_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("id_power")
            .child(
                Icon::new(IconName::Power)
                    .text_color(rgb(0xF4F4F4))
                    .size((px(24.), px(24.))),
            )
            .on_click(cx.listener(
                move |_, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>| {
                    println!("power clicked");
                    cx.notify();
                },
            ))
    }

    fn render_rotation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let rotation_icon = if self.rotation_on {
            IconName::RotationOn
        } else {
            IconName::RotationOff
        };

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
            ))
    }

    fn render_airplane_mode(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            ))
    }

    fn render_screen_mirroring(&self, cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: Add extened screen icons when extended screen is detected
        let screen_mirroring_icon = if self.screen_mirroring {
            IconName::ScreenMirroringOn
        } else {
            IconName::ScreenMirroringOff
        };

        IconButton::new("id_screen_mirroring")
            .icon(screen_mirroring_icon)
            .size((px(104.), px(88.)))
            .active(self.screen_mirroring)
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            // .on_click(cx.listener( // keep this
            //     |this: &mut SettingsDrawer,
            //      _event: &MouseUpEvent,
            //      _window: &mut Window,
            //      cx: &mut Context<Self>| {
            //         this.screen_mirroring = !this.screen_mirroring;
            //         cx.notify();
            //     },
            // ))
            .on_click(cx.listener(
                // TODO: add long press
                move |this: &mut SettingsDrawer,
                      _event: &ClickEvent,
                      _window: &mut Window,
                      cx: &mut Context<Self>| {
                    this.current_modal = ModalKind::ScreenMirroring;
                    Self::start_animation(this, _event, _window, cx);

                    cx.notify();
                },
            ))
    }

    fn render_terminal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        IconButton::new("id_terminal")
            .icon(IconName::Terminal)
            .size((px(104.), px(88.)))
            .icon_color(rgb(DARK_NEUTRAL_100))
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            .on_click(cx.listener(
                |_, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>| {
                    println!("terminal clicked");
                    cx.notify();
                },
            ))
    }

    fn render_microphone_recording(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            ))
    }

    fn render_screen_recording(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            ))
    }

    fn render_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        IconButton::new("id_settings")
            .icon(IconName::Settings)
            .size((px(104.), px(88.)))
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            .on_click(cx.listener(
                |_, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>| {
                    println!("settigns clicked");
                    cx.notify();
                },
            ))
    }

    fn render_camera(&self, cx: &mut Context<Self>) -> impl IntoElement {
        IconButton::new("id_camera")
            .icon(IconName::CameraOff)
            .size((px(104.), px(88.)))
            // .icon_color(rgb(AMBER_600)) // on press change ICON CameraON
            .icon_color(rgb(DARK_NEUTRAL_100))
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            .on_click(cx.listener(
                |_, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>| {
                    println!("camera clicked");
                    cx.notify();
                },
            ))
    }

    fn render_wireless(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut wireless_icon = IconName::WifiOff;
        let mut network_label = "Wi-Fi".to_string();
        let wireless_connected_network = self.wireless_details.connected_network.clone();
        let wireless_enable = self.wireless_details.enabled;
        if wireless_enable && wireless_connected_network.is_some() {
            network_label = wireless_connected_network
                .clone()
                .map(|s| s.ssid)
                .unwrap_or_else(|| "Wi-Fi".to_string());

            wireless_icon = if network_label == "Wi-Fi" {
                IconName::ConnectedWifiOn
            } else {
                let signal_strength = wireless_connected_network
                    .clone()
                    .map(|info| info.signal_strength)
                    .unwrap_or_else(|| 0);
                get_wireless_strength_icon(wireless_enable, signal_strength, "Open".to_string()) // intentionally open as no lock to show in view
            };
        }

        IconButton::new("id_wireless")
            .icon(Icon::new(wireless_icon).size((px(36.), px(36.))))
            .size((px(104.), px(104.)))
            .active(self.wireless_details.enabled)
            .label(network_label)
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            // // // 10% - ok - keeping while it is active
            .on_click(cx.listener(
                // keep this - quick click
                |this: &mut SettingsDrawer,
                 _event: &ClickEvent,
                 _window: &mut Window,
                 cx: &mut Context<Self>| {
                    let shell_state = ShellState::global(cx).clone();
                    let is_enable = this.wireless_details.enabled;
                    this.wireless_details.enabled = !is_enable;
                    cx.background_executor()
                        .spawn(async move {
                            shell_state.toggle_wireless(!is_enable).await;
                        })
                        .detach();
                    cx.notify();
                },
            ))
        // .on_click(cx.listener(
        //     // TODO: long press open modal
        //     move |this: &mut SettingsDrawer,
        //           _event: &ClickEvent,
        //           window: &mut Window,
        //           cx: &mut Context<Self>| {
        //         this.current_modal = ModalKind::WirelessModal;
        //         Self::start_animation(this, _event, window, cx);
        //     },
        // ))
    }

    fn render_bluetooth(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let bluetooth_icon = match self.bluetooth_details.enabled {
            true => match self.bluetooth_details.connected_devices > 0 {
                true => IconName::BluetoothConnected,
                false => IconName::BluetoothOn,
            },
            false => IconName::BluetoothOff,
        };
        let bluetooth_label = match self.bluetooth_details.enabled {
            true => {
                if self.bluetooth_details.connected_devices == 0 {
                    "Bluetooth".to_string()
                } else {
                    format!("{} Devices", self.bluetooth_details.connected_devices)
                }
            }
            false => "Bluetooth".to_string(),
        };

        IconButton::new("id_bluetooth")
            .icon(bluetooth_icon)
            .size((px(104.), px(104.)))
            .label(bluetooth_label)
            .active(self.bluetooth_details.enabled)
            .active_icon_color(rgb(AMBER_600))
            .active_bg_color(rgba(AMBER_600_10))
            .on_click(cx.listener(
                |this: &mut SettingsDrawer,
                 _event: &ClickEvent,
                 _window: &mut Window,
                 cx: &mut Context<Self>| {
                    let shell_state = ShellState::global(cx).clone();
                    let is_enable = this.bluetooth_details.enabled;
                    this.bluetooth_details.enabled = !is_enable;
                    cx.background_executor()
                        .spawn(async move {
                            shell_state.toggle_bluetooth(!is_enable).await;
                        })
                        .detach();
                    cx.notify();
                },
            ))
        // .on_click(cx.listener(
        //     // TODO: add long press open modal
        //     move |this: &mut SettingsDrawer,
        //           _event: &ClickEvent,
        //           _window: &mut Window,
        //           cx: &mut Context<Self>| {
        //         this.current_modal = ModalKind::BluetoothModal;
        //         Self::start_animation(this, _event, _window, cx);
        //     },
        // ))
    }

    fn render_battery_performance(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let power_mode_icon = match self.power_mode {
            PowerMode::High => IconName::PowerModeHigh,
            PowerMode::Balanced => IconName::PowerModeBalanced,
            PowerMode::Low => IconName::PowerModeLow,
        };
        let power_mode_icon_color = match self.power_mode {
            PowerMode::High => rgb(AMBER_600),
            PowerMode::Balanced => rgb(DARK_NEUTRAL_100),
            PowerMode::Low => rgb(AMBER_600),
        };
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
                    this.current_modal = ModalKind::PerformanceModal;
                    Self::start_animation(this, _event, _window, cx);
                },
            ))
    }

    fn render_cell_signal(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            ))
    }

    fn render_brightness_control_div(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            .when(!self.open_modal, |this| {
                this.on_click(cx.listener(
                    // CHECK - on long press
                    move |this: &mut SettingsDrawer,
                          _event: &ClickEvent,
                          _window: &mut Window,
                          cx: &mut Context<Self>| {
                        this.current_modal = ModalKind::DisplayModal;
                        Self::start_animation(this, _event, _window, cx);
                    },
                ))
            })
            .child(self.render_brightness_slider(cx, 167.0))
    }

    pub fn render_brightness_slider(&self, cx: &mut Context<Self>, width: f32) -> impl IntoElement {
        let brightness_icon =
            if self.brightness_slider_value >= 0.0 && self.brightness_slider_value <= 33.0 {
                IconName::BrightnessLow
            } else if self.brightness_slider_value > 33.0 && self.brightness_slider_value <= 66.0 {
                IconName::BrightnessMedium
            } else {
                IconName::BrightnessHigh
            };
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_start()
            .pl_2()
            .child(
                IconButton::new("id_brightness")
                    .icon(brightness_icon)
                    .icon_color(rgb(AMBER_600))
                    .size((px(32.), px(32.)))
                    .bg_color(rgb(DARK_NEUTRAL_900))
                    .active_bg_color(rgb(DARK_NEUTRAL_900))
                    .border(px(0.)),
            )
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .w(px(width))
                    .pl_2()
                    .child(
                        Slider::new("brightness-slider", &self.brightness_slider_state)
                            .height(66.0)
                            .width(width),
                    ),
            )
    }

    fn render_volume_control(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            DARK_NEUTRAL_0
        } else {
            AMBER_600
        };

        div()
            .id("id_volume")
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_start()
            .pl_2()
            .on_click(cx.listener(
                //  TODO: long press open modal
                move |this: &mut SettingsDrawer,
                      _event: &ClickEvent,
                      _window: &mut Window,
                      cx: &mut Context<Self>| {
                    println!("volume clicked");
                    this.current_modal = ModalKind::SoundModal;
                    Self::start_animation(this, _event, _window, cx);
                },
            ))
            .child(
                IconButton::new("id_mute_volume")
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
                            let mut volume_tx = this.volume_tx.clone().unwrap();

                            this.volume_mute = !this.volume_mute;
                            let is_mute = this.volume_mute;
                            let sink_name = this
                                .volume_device_name
                                .clone()
                                .unwrap_or_else(|| "default".to_string());

                            if is_mute {
                                cx.background_executor()
                                    .spawn(async move {
                                        let _ = volume_tx
                                            .send(VolumeMessage::MuteSink {
                                                name: sink_name.clone(),
                                            })
                                            .await;
                                    })
                                    .detach();
                            } else {
                                cx.background_executor()
                                    .spawn(async move {
                                        let _ = volume_tx
                                            .send(VolumeMessage::UnmuteSink {
                                                name: sink_name.clone(),
                                            })
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
                    .child(Slider::new("volume-slider", &self.volume_slider_state).height(66.0)),
            )
    }
}
