mod modals;
mod widgets;
use commons::widgets::{WingSide, wing};
use dispatcher::{Dispatcher, Message};
use gpui::prelude::FluentBuilder;
use icons::prelude::{Icons, SettingsDrawerIcons};
use mxsearch::prelude::AppInfo;
use mxsearch::service::MxSearchService;
use settings::prelude::{Settings, SettingsDrawerSettings};
use shell_state::DEFAULT_MIN_BRIGHTNESS;
use shell_state::{BrightnessMessage, ShellState, VolumeMessage};
use theme::prelude::AlphaExt;

use crate::helper::get_wireless_strength_icon;
use crate::ui::modals::{
    bluetooth_modal::BluetoothModalScroll, wireless_modal::WirelessModalScroll,
};
use crate::ui::widgets::{IconButton, Slider, SliderEvent, SliderState};
use futures::SinkExt;
use gpui::*;
use theme::ActiveTheme;

const MIN_MODAL_SIZE_1: (f32, f32) = (133., 110.);
const MIN_MODAL_SIZE_2: (f32, f32) = (203., 168.);
pub const FINAL_MODAL_SIZE: (f32, f32) = (478., 392.);

const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 4;
const ICON_W: f32 = 104.0;
const ICON_H: f32 = 104.0;
const ROW_12_ICON_H: f32 = 88.0;

const ANIMATION_DURATION_MS: f32 = 250.0;
const ANIMATION_FRAME_MS: u64 = 16;

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

pub struct SettingsDrawer {
    pub open_power_options: bool,

    pub rotation_on: bool,
    pub airplane_mode: bool,
    pub screen_mirroring: bool,

    pub microphone_recording: bool,
    pub screen_recording: bool,
    pub power_mode: PowerMode,
    pub cell_signal: bool,

    pub brightness_slider_state: Entity<SliderState>,
    pub brightness_slider_value: f32,
    pub auto_brightness: bool,
    pub dark_mode: bool,

    pub volume_slider_state: Entity<SliderState>,
    pub volume_slider_value: f32,
    pub volume_device_name: Option<String>,
    pub volume_mute: bool,

    pub open_modal: bool,
    pub animation_progress: f32,
    pub animation_state: ModalAnimationState,
    pub modal_size: (f32, f32),

    pub modal_start_center: (f32, f32),
    pub modal_origin_center: (f32, f32),
    pub modal_current_center: (f32, f32),
    pub modal_target_center: (f32, f32),

    pub current_modal: ModalKind,

    _subscriptions: Vec<Subscription>,
    pub position: f32,
    drag_offset: Option<f32>,
    drag_start_pos: f32,
    pub is_visible: bool,

    pub wireless_modal_scroll: WirelessModalScroll,
    pub bluetooth_modal_scroll: BluetoothModalScroll,

    pub settings_app_info: Option<AppInfo>,
    pub camera_app_info: Option<AppInfo>,
    pub terminal_info: Option<AppInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let settings = Settings::global(cx).settings_drawer.clone();
        let apps = Settings::global(cx).settings_drawer.clone().drawer_apps;

        let ShellState {
            volume_tx,
            brightness_tx,
            ..
        } = ShellState::global(cx).clone();

        let brightness_slider = cx.new(|_| SliderState::new());
        let volume_slider = cx.new(|_| {
            SliderState::new()
                .default_value(0.)
                .pattern(widgets::SliderPattern::Bars)
        });

        let b_subscription =
            Self::create_brightness_subscription(&brightness_slider, brightness_tx, cx);
        let v_subscription = Self::create_volume_subscription(&volume_slider, volume_tx, cx);

        let mut _subscriptions = vec![b_subscription, v_subscription];

        let closed_pos = Self::calculate_closed_position(&settings);

        let settings_drawer = Self {
            open_power_options: false,
            rotation_on: false,
            airplane_mode: false,
            screen_mirroring: false,
            power_mode: PowerMode::Balanced,
            microphone_recording: false,
            screen_recording: false,

            volume_device_name: None,
            cell_signal: false,
            brightness_slider_state: brightness_slider,
            brightness_slider_value: 0.0,
            auto_brightness: false,
            dark_mode: false,

            volume_slider_state: volume_slider,
            volume_slider_value: 0.0,
            volume_mute: false,
            open_modal: false,
            animation_progress: 0.0,
            animation_state: ModalAnimationState::None,
            modal_size: (MIN_MODAL_SIZE_1.0, MIN_MODAL_SIZE_1.1),
            modal_start_center: (0.0, 0.0),
            modal_origin_center: (0.0, 0.0),
            modal_current_center: (0.0, 0.0),
            modal_target_center: (0.0, 0.0),

            _subscriptions,
            current_modal: ModalKind::None,
            position: closed_pos,
            drag_offset: None,
            drag_start_pos: 0.0,
            is_visible: false,

            wireless_modal_scroll: WirelessModalScroll::new(),
            bluetooth_modal_scroll: BluetoothModalScroll::new(),
            settings_app_info: None,
            camera_app_info: None,
            terminal_info: None,
        };

        cx.spawn(async move |this, cx| {
            if let Ok(service) = MxSearchService::new().await {
                let mut settings_app: Option<AppInfo> = None;
                let mut camera_app: Option<AppInfo> = None;
                let mut terminal_app: Option<AppInfo> = None;
                if let Ok(apps) = service.search_applications(&apps.settings).await {
                    settings_app = apps.first().cloned();
                }

                if let Ok(apps) = service.search_applications(&apps.camera).await {
                    camera_app = apps.first().cloned();
                }

                if let Ok(apps) = service.search_applications(&apps.terminal).await {
                    terminal_app = apps.first().cloned();
                }

                this.update(cx, |this, cx| {
                    this.settings_app_info = settings_app;
                    this.camera_app_info = camera_app;
                    this.terminal_info = terminal_app;

                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
        settings_drawer
    }

    pub fn calculate_closed_position(settings: &SettingsDrawerSettings) -> f32 {
        let closed_pos_px = Self::closed_pos(settings.layer_shell.size, settings.navbar_size);
        closed_pos_px.into()
    }

    fn create_brightness_subscription(
        brightness_slider: &Entity<SliderState>,
        brightness_tx: Option<futures::channel::mpsc::Sender<BrightnessMessage>>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(
            brightness_slider,
            move |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                this.brightness_slider_value = *value;

                if let Some(mut tx) = brightness_tx.clone() {
                    let brightness_value = *value;
                    cx.background_executor()
                        .spawn(async move {
                            let _ = tx
                                .send(BrightnessMessage::BrightnessChanged {
                                    value: brightness_value,
                                })
                                .await;
                        })
                        .detach();
                }

                let clamped_value = value.max(DEFAULT_MIN_BRIGHTNESS);
                this.brightness_slider_state.update(cx, |state, _cx| {
                    state.value = clamped_value.clamp(state.min, state.max);
                });

                cx.notify();
            },
        )
    }

    fn create_volume_subscription(
        volume_slider: &Entity<SliderState>,
        volume_tx: Option<futures::channel::mpsc::Sender<VolumeMessage>>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(volume_slider, move |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.volume_slider_value = *value;

            if let Some(mut tx) = volume_tx.clone() {
                let sink_name = this
                    .volume_device_name
                    .clone()
                    .unwrap_or_else(|| "default".to_string());
                let volume = *value;

                cx.background_executor()
                    .spawn(async move {
                        let _ = tx
                            .send(VolumeMessage::VolumeChanged {
                                name: sink_name,
                                value: volume,
                            })
                            .await;
                    })
                    .detach();
            }

            this.volume_mute = *value <= 0.0;
            this.volume_slider_value = if this.volume_mute { 0.0 } else { *value };
            this.volume_slider_state.update(cx, |state, _cx| {
                state.value = value.clamp(state.min, state.max);
            });

            cx.notify();
        })
    }

    fn start_animation(&mut self, event: &LongPressEvent, _: &mut Window, cx: &mut Context<Self>) {
        let settings = Settings::global(cx).settings_drawer.clone();
        let settings_drawer_size = settings.layer_shell.size;

        let position = event.current_position;

        // Start from clicked icon center
        self.modal_start_center =
            Self::clicked_item_center(position.x.into(), position.y.into(), cx);
        self.modal_origin_center = self.modal_start_center;

        self.modal_current_center = self.modal_start_center;

        // End at APP center
        self.modal_target_center = (
            settings_drawer_size.width / px(2.0),
            settings_drawer_size.height / px(2.0),
        );

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

    fn clicked_item_center(mouse_x: f32, mouse_y: f32, cx: &mut Context<Self>) -> (f32, f32) {
        let settings = Settings::global(cx).settings_drawer.clone();
        let settings_drawer_size = settings.layer_shell.size;

        let grid_w = GRID_COLS as f32 * ICON_W;
        let grid_h = GRID_ROWS as f32 * ICON_H;

        let origin_x = (settings_drawer_size.width - px(grid_w)) / px(2.0);
        let origin_y = (settings_drawer_size.height - px(grid_h)) / px(2.0);

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
        let settings = Settings::global(cx).settings_drawer.clone();
        let navbar_size = settings.navbar_size;
        let closed_pos_f32: f32 = Self::calculate_closed_position(&settings);

        let colors = cx.theme().colors.clone();

        let open_y = 0.;
        let closed_y = closed_pos_f32;

        let threshold_px = 40.;
        self.update_input_regions(self.is_visible, window, cx);

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
                            } else {
                                target = closed_y;
                            }
                        } else {
                            if this.position > (open_y + threshold_px) {
                                target = closed_y;
                            } else {
                                target = open_y;
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
                            .h(navbar_size.height)
                            .child(
                                div()
                                    .id("right-wing")
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(
                                            navbar_size.width,
                                            navbar_size.height,
                                        ));
                                        w.upper_wing_side(WingSide::Right);
                                        w.w(navbar_size.width).h(navbar_size.height).bg(
                                            if self.is_visible {
                                                colors.background_1000
                                            } else {
                                                colors.background_800
                                            },
                                        )
                                    })
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
                    .when(self.is_visible, |content_div| {
                        content_div.size_full().bg(colors.background_1000)
                    })
                    .child(self.drawer_items(window, cx)),
            )
    }
}

impl SettingsDrawer {
    pub fn closed_pos(app_size: Size<Pixels>, navbar_size: Size<Pixels>) -> Pixels {
        app_size.height - navbar_size.height
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let settings = Settings::global(cx).settings_drawer.clone();
        let closed_pos = Self::calculate_closed_position(&settings);

        let start = self.position;
        let change = target - start;
        let start_time = std::time::Instant::now();

        if target == 0.0 {
            self.is_visible = true;
        } else if target == closed_pos {
            self.is_visible = false;
        }

        cx.spawn(
            async move |this: WeakEntity<SettingsDrawer>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    // Check if animation is done
                    if elapsed >= ANIMATION_DURATION_MS {
                        this.update(cx, |this, cx| {
                            this.position = target;
                            if target == 0.0 {
                                this.is_visible = true;
                            } else if target == closed_pos {
                                this.is_visible = false;
                            }
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / ANIMATION_DURATION_MS).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position = current;
                        if current < (closed_pos / 2.0) {
                            this.is_visible = true;
                        } else {
                            this.is_visible = false;
                        }
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(ANIMATION_FRAME_MS))
                        .await;
                }
            },
        )
        .detach();
    }
    fn update_input_regions(&self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        let mut regions = Vec::new();

        let settings = Settings::global(cx).settings_drawer.clone();
        let navbar_size = settings.navbar_size;
        let settings_drawer_size = settings.layer_shell.size;
        let closed_pos_px = Self::closed_pos(settings_drawer_size, navbar_size);

        if open {
            regions.push(Bounds {
                origin: point(px(0.), px(0.)),
                size: settings_drawer_size,
            });
        } else {
            regions.push(Bounds {
                origin: point(
                    settings_drawer_size.width - navbar_size.width,
                    closed_pos_px,
                ),
                size: navbar_size,
            });
        }
        window.set_input_regions(Some(regions));
        cx.notify();
    }

    fn drawer_items(
        &mut self,
        window: &mut Window,
        cx: &mut Context<SettingsDrawer>,
    ) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let current_time_date = ShellState::global(cx).current_time_date.clone();

        let settings = Settings::global(cx).settings_drawer.clone();
        let navbar_size = settings.navbar_size;
        let settings_drawer_size = settings.layer_shell.size;

        if matches!(
            self.animation_state,
            ModalAnimationState::Opening | ModalAnimationState::Closing
        ) {
            match self.animation_state {
                ModalAnimationState::Opening => {
                    self.animation_progress += 0.10;
                }
                ModalAnimationState::Closing => {
                    self.animation_progress -= 0.10;
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
            .w(settings_drawer_size.width)
            .h(settings_drawer_size.height)
            .child(
                div()
                    .id("main_container")
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .px_8()
                    .bg(colors.background_1000)
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
                                    .child(current_time_date)
                                    .text_xl()
                                    .text_color(colors.foreground_100),
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
                            .child(self.render_sound_control_div(cx)),
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
                                    .bg(colors.background_900)
                                    .opacity(0.6)
                                    .on_click(cx.listener(|this: &mut SettingsDrawer, _, _, cx| {
                                        Self::start_close_animation(this, cx);
                                    }))
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
                                                        let shell_state =
                                                            ShellState::global(cx).clone();

                                                        cx.background_executor()
                                                            .spawn(async move {
                                                                shell_state
                                                                    .get_output_sound_devices()
                                                                    .await;
                                                            })
                                                            .detach();

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

    fn launch_app(app_info: Option<AppInfo>, cx: &mut Context<Self>) {
        if app_info.is_none() {
            return;
        }
        let sender = Dispatcher::global(cx).0.clone();
        let app_info = app_info.clone().unwrap();

        cx.background_executor()
            .spawn(async move {
                _ = sender
                    .broadcast(dispatcher::Message::LaunchApp {
                        app_id: app_info.possible_app_id,
                        exec: app_info.exec,
                    })
                    .await;
            })
            .detach();
    }

    fn open_modal_on_long_press(
        modal: ModalKind,
        is_enabled: bool,
    ) -> impl Fn(&mut Self, &LongPressEvent, &mut Window, &mut Context<Self>) + Clone {
        move |this, event, window, cx| {
            if !is_enabled {
                return;
            }

            if !this.open_modal {
                this.current_modal = modal;
                Self::start_animation(this, event, window, cx);
            } else {
                this.open_modal = false;
            }
        }
    }

    fn render_power_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let power_off = Icons::global(cx).settings_drawer.power_off.clone();

        div()
            .id("id_power")
            .child(
                svg()
                    .external_path(SharedString::from(power_off.to_string_lossy().to_string()))
                    .text_color(colors.foreground_100)
                    .w(px(24.))
                    .h(px(24.)),
            )
            .on_click(cx.listener(
                move |_, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>| {
                    println!("power clicked");
                    let dispatcher_tx = Dispatcher::global(cx).channel().0.clone();

                    println!("checking dispatcher {} ", dispatcher_tx.is_empty());

                    // here send message to show power options
                    cx.background_executor()
                        .spawn(async move {
                            let _ = dispatcher_tx
                                .broadcast(Message::ShowPowerOptions(true))
                                .await;
                        })
                        .detach();

                    // cx.background_executor()
                    //     .spawn(async move {
                    //         let _ = dispatcher_tx.send(Message::ShowPowerOptions(true));
                    //     })
                    //     .detach();

                    cx.notify();
                },
            ))
    }

    fn render_rotation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let rotation_on = Icons::global(cx).settings_drawer.rotation_on.clone();
        let rotation_off = Icons::global(cx).settings_drawer.rotation_off.clone();
        let rotation_icon = if self.rotation_on {
            rotation_on
        } else {
            rotation_off
        };

        IconButton::new("id_rotation")
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                rotation_icon.to_string_lossy().to_string(),
            )))
            .icon_color(colors.foreground_600)
            .active(self.rotation_on)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
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
        let colors = cx.theme().colors.clone();
        let airplane = Icons::global(cx).settings_drawer.airplane.clone();

        IconButton::new("id_airplane")
            .icon(
                svg()
                    .size(px(36.))
                    .external_path(SharedString::from(airplane.to_string_lossy().to_string())),
            )
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .icon_color(colors.foreground_600)
            .active(self.airplane_mode)
            .active_icon_color(colors.foreground_0)
            .active_bg_color(colors.accent_200)
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
        let colors = cx.theme().colors.clone();
        let screen_mirroring_on = Icons::global(cx)
            .settings_drawer
            .screen_mirroring_on
            .clone();
        let screen_mirroring_off = Icons::global(cx)
            .settings_drawer
            .screen_mirroring_off
            .clone();
        // TODO: Add extended screen icons when extended screen is detected
        let screen_mirroring_icon = if self.screen_mirroring {
            screen_mirroring_on
        } else {
            screen_mirroring_off
        };

        IconButton::new("id_screen_mirroring")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                screen_mirroring_icon.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .active(self.screen_mirroring)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(cx.listener(
                |this: &mut SettingsDrawer,
                 _event: &ClickEvent,
                 _window: &mut Window,
                 cx: &mut Context<Self>| {
                    this.screen_mirroring = !this.screen_mirroring;
                    cx.notify();
                },
            ))
            .on_long_press(cx.listener(Self::open_modal_on_long_press(
                ModalKind::ScreenMirroring,
                true,
            )))
    }

    fn render_terminal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let terminal = Icons::global(cx).settings_drawer.terminal.clone();

        IconButton::new("id_terminal")
            .icon(
                svg()
                    .size(px(36.))
                    .external_path(SharedString::from(terminal.to_string_lossy().to_string())),
            )
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .icon_color(colors.foreground_600)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(cx.listener(
                |this, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                    Self::launch_app(this.terminal_info.clone(), cx);
                    cx.notify();
                },
            ))
    }

    fn render_microphone_recording(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let microphone_off = Icons::global(cx).settings_drawer.microphone_off.clone();
        IconButton::new("id_microphone")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                microphone_off.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .active(self.microphone_recording)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
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
        let colors = cx.theme().colors.clone();
        let screen_recording_off = Icons::global(cx)
            .settings_drawer
            .screen_recording_off
            .clone();

        IconButton::new("id_screen_recording")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                screen_recording_off.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .active(self.screen_recording)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
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
        let colors = cx.theme().colors.clone();
        let settings = Icons::global(cx).settings_drawer.settings.clone();

        IconButton::new("id_settings")
            .icon(
                svg()
                    .size(px(36.))
                    .external_path(SharedString::from(settings.to_string_lossy().to_string())),
            )
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(cx.listener(
                |this, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                    Self::launch_app(this.settings_app_info.clone(), cx);
                    cx.notify();
                },
            ))
    }

    fn render_camera(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let camera_off = Icons::global(cx).settings_drawer.camera_off.clone();

        IconButton::new("id_camera")
            .icon(
                svg()
                    .size(px(36.))
                    .external_path(SharedString::from(camera_off.to_string_lossy().to_string())),
            )
            .size((px(ICON_W), px(ROW_12_ICON_H)))
            .icon_color(colors.foreground_600)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(cx.listener(
                |this, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                    Self::launch_app(this.camera_app_info.clone(), cx);
                    cx.notify();
                },
            ))
    }

    fn render_wireless(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let wireless_off = Icons::global(cx).settings_drawer.wireless_off.clone();
        let connected_wireless_on = Icons::global(cx)
            .settings_drawer
            .connected_wireless_on
            .clone();
        let wireless_details = ShellState::global(cx).wireless_details.clone();
        let (wireless_icon, network_label) = {
            let mut icon = wireless_off;
            let mut label = "Wi-Fi".to_string();

            if wireless_details.enabled {
                if let Some(ref network) = wireless_details.connected_network {
                    label = network.ssid.clone();

                    icon = if label == "Wi-Fi" {
                        connected_wireless_on
                    } else {
                        get_wireless_strength_icon(
                            true,
                            network.signal_strength,
                            "Open".to_string(), // intentional: show open in view
                            cx,
                        )
                    };
                }
            }

            (icon, label)
        };

        IconButton::new("id_wireless")
            .icon(
                svg()
                    .external_path(SharedString::from(
                        wireless_icon.to_string_lossy().to_string(),
                    ))
                    .size(px(36.)),
            )
            .size((px(ICON_W), px(ICON_H)))
            .active(wireless_details.enabled)
            .label(network_label)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(
                cx.listener(move |_: &mut SettingsDrawer, _: &ClickEvent, _window, cx| {
                    let shell_state = ShellState::global(cx).clone();
                    let is_enabled_now = ShellState::global(cx).wireless_details.enabled;
                    cx.background_executor()
                        .spawn(async move {
                            shell_state.toggle_wireless(!is_enabled_now).await;
                        })
                        .detach();

                    cx.notify();
                }),
            )
            .on_long_press(cx.listener(Self::open_modal_on_long_press(
                ModalKind::WirelessModal,
                wireless_details.enabled,
            )))
    }

    fn render_bluetooth(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let bluetooth_details = ShellState::global(cx).bluetooth_details.clone();
        let SettingsDrawerIcons {
            bluetooth_off,
            bluetooth_connected,
            bluetooth_on,
            ..
        } = Icons::global(cx).settings_drawer.clone();

        let (bluetooth_icon, bluetooth_label) = {
            let enabled = bluetooth_details.enabled;
            let connected = bluetooth_details.connected_devices;

            let icon = if !enabled {
                bluetooth_off
            } else if connected > 0 {
                bluetooth_connected
            } else {
                bluetooth_on
            };

            let label = if enabled && connected > 0 {
                format!("{} Devices", connected)
            } else {
                "Bluetooth".to_string()
            };

            (icon, label)
        };

        IconButton::new("id_bluetooth")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                bluetooth_icon.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ICON_H)))
            .label(bluetooth_label)
            .active(bluetooth_details.enabled)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(
                cx.listener(move |_: &mut SettingsDrawer, _: &ClickEvent, _window, cx| {
                    let shell_state = ShellState::global(cx).clone();
                    let is_enabled_now = ShellState::global(cx).bluetooth_details.enabled;
                    cx.background_executor()
                        .spawn(async move {
                            shell_state.toggle_bluetooth(!is_enabled_now).await;
                        })
                        .detach();
                    cx.notify();
                }),
            )
            .on_long_press(cx.listener(Self::open_modal_on_long_press(
                ModalKind::BluetoothModal,
                bluetooth_details.enabled,
            )))
    }

    fn render_battery_performance(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let battery_percent = ShellState::global(cx).battery_percent.clone();
        let SettingsDrawerIcons {
            power_mode_high,
            power_mode_balanced,
            power_mode_low,
            ..
        } = Icons::global(cx).settings_drawer.clone();

        let power_mode_icon = match self.power_mode {
            PowerMode::High => power_mode_high,
            PowerMode::Balanced => power_mode_balanced,
            PowerMode::Low => power_mode_low,
        };
        let power_mode_icon_color = match self.power_mode {
            PowerMode::High => colors.accent_200,
            PowerMode::Balanced => colors.foreground_600,
            PowerMode::Low => colors.accent_200,
        };
        IconButton::new("id_power_mode")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                power_mode_icon.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ICON_H)))
            .label(format!("{}% ", battery_percent))
            .icon_color(power_mode_icon_color)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
            .on_click(cx.listener(
                move |this: &mut SettingsDrawer,
                      _event: &ClickEvent,
                      _window: &mut Window,
                      cx: &mut Context<Self>| {
                    // TODO: set power saving mode on click
                    cx.notify();
                },
            ))
            .on_long_press(cx.listener(Self::open_modal_on_long_press(
                ModalKind::PerformanceModal,
                true,
            )))
    }

    fn render_cell_signal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let cell_signal_none = Icons::global(cx).settings_drawer.cell_signal_none.clone();
        IconButton::new("id_cell_signal")
            .icon(svg().size(px(36.)).external_path(SharedString::from(
                cell_signal_none.to_string_lossy().to_string(),
            )))
            .size((px(ICON_W), px(ICON_H)))
            .label("No SIM")
            .active(self.cell_signal)
            .active_icon_color(colors.accent_200)
            .active_bg_color(colors.accent_200.with_alpha(0.1))
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
        let colors = cx.theme().colors.clone();

        div()
            .id("id_display")
            .flex()
            .items_center()
            .w_full()
            .h_full()
            .text_lg()
            .col_span(2)
            .bg(colors.background_900)
            .rounded(px(8.))
            .on_click(cx.listener(|_, _: &ClickEvent, _, _| {}))
            .on_long_press(cx.listener(Self::open_modal_on_long_press(
                ModalKind::DisplayModal,
                true,
            )))
            .child(self.render_brightness_slider(cx, 167.0))
    }

    pub fn render_brightness_slider(&self, cx: &mut Context<Self>, width: f32) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let SettingsDrawerIcons {
            brightness_low,
            brightness_medium,
            brightness_high,
            ..
        } = Icons::global(cx).settings_drawer.clone();
        let brightness_icon =
            if self.brightness_slider_value >= 0.0 && self.brightness_slider_value <= 33.0 {
                brightness_low
            } else if self.brightness_slider_value > 33.0 && self.brightness_slider_value <= 66.0 {
                brightness_medium
            } else {
                brightness_high
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
                    .icon(svg().size(px(36.)).external_path(SharedString::from(
                        brightness_icon.to_string_lossy().to_string(),
                    )))
                    .icon_color(colors.accent_200)
                    .size((px(32.), px(32.)))
                    .bg_color(colors.background_900)
                    .active_bg_color(colors.background_900)
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

    fn render_sound_control_div(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();

        div()
            .id("id_sound")
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .text_lg()
            .col_span(2)
            .bg(colors.background_900)
            .rounded(px(8.))
            .on_click(cx.listener(|_, _, _, _cx: &mut Context<Self>| {}))
            .on_long_press(cx.listener(Self::open_modal_on_long_press(ModalKind::SoundModal, true)))
            .child(self.render_volume_slider(cx))
    }

    fn render_volume_slider(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let SettingsDrawerIcons {
            volume_off,
            volume_low,
            volume_medium,
            volume_high,
            ..
        } = Icons::global(cx).settings_drawer.clone();

        let volume_icon = if self.volume_mute {
            volume_off
        } else {
            if self.volume_slider_value >= 0.0 && self.volume_slider_value <= 33.0 {
                volume_low
            } else if self.volume_slider_value > 33.0 && self.volume_slider_value <= 66.0 {
                volume_medium
            } else {
                volume_high
            }
        };
        let volume_icon_color = if self.volume_mute {
            colors.foreground_0
        } else {
            colors.accent_200
        };

        let volume_tx = ShellState::global(cx).volume_tx.clone().unwrap();
        let colors = cx.theme().colors.clone();

        div()
            .id("id_volume")
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_start()
            .pl_2()
            .child(
                div()
                    .id("volume_icon")
                    .bg(colors.background_900)
                    .child(
                        svg()
                            .external_path(SharedString::from(
                                volume_icon.to_string_lossy().to_string(),
                            ))
                            .text_color(volume_icon_color)
                            .size(px(32.)),
                    )
                    .on_click(cx.listener(
                        move |this: &mut SettingsDrawer,
                              _event: &ClickEvent,
                              _window: &mut Window,
                              cx: &mut Context<Self>| {
                            let mut volume_tx = volume_tx.clone();

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
