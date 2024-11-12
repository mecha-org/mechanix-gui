use crate::modules::cpu::component::GRID_SIZE;
use crate::modules::installed_apps;
use crate::modules::running_apps::running_app::{AppDetails, RunningApp};
use crate::modules::settings_panel::rotation::component::RotationStatus;
use crate::pages::app_drawer::AppDrawer;
use crate::pages::app_switcher::AppSwitcher;
use crate::pages::home_ui::HomeUi;
use crate::pages::power_options::PowerOptions;
use crate::pages::settings_panel::SettingsPanel;
use crate::pages::status_bar::StatusBar;
use crate::settings::{self, LauncherSettings};
use crate::theme::{self, LauncherTheme};
use crate::types::{BatteryLevel, BluetoothStatus, RestartState, ShutdownState, WirelessStatus};
use crate::utils::{cubic_bezier, get_formatted_battery_level};
use crate::{
    AppMessage, AppParams, BluetoothMessage, BrightnessMessage, SoundMessage, WirelessMessage,
};
use command::spawn_command;
use desktop_entries::DesktopEntry;
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::reexports::smithay_client_toolkit::shell::wlr_layer::Layer;
use mctk_core::renderables::rect::InstanceBuilder;
use mctk_core::renderables::{Image, Rect, Renderable};
use mctk_core::{component, msg, Color, Point, Pos, Scale, AABB};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};
use std::any::Any;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use upower::BatteryStatus;
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;

pub const BEZIER_POINTS: [f64; 4] = [0.0, 0.0, 480.0, 480.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screens {
    #[default]
    Home,
    AppDrawer,
    RunningApps,
    Notification,
}

#[derive(Debug, Clone)]
pub enum SettingNames {
    Wireless,
    Bluetooth,
    Rotation,
    Terminal,
    Power,
}
#[derive(Debug, Clone)]
pub enum SliderSettingsNames {
    Brightness { value: u8 },
    Sound { value: u8 },
    Lock { value: u8 },
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppClicked { app_id: String },
    Clock { date: String, time: String },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
    SettingClicked(SettingNames),
    SliderChanged(SliderSettingsNames),
    CPUUsage { usage: f32 },
    Uptime { uptime: String },
    MachineName { name: String },
    IpAddress { address: String },
    Net { online: bool },
    Memory { total: u64, used: u64 },
    Swipe { swipe: Swipe },
    AppListAppClicked { app: DesktopEntry },
    SwipeEnd,
    Sound { value: u8 },
    Brightness { value: u8 },
    RunningApps { count: i32 },
    Unlock,
    AppsUpdated { apps: Vec<AppDetails> },
    AppInstanceClicked(ToplevelKey),
    AppInstanceCloseClicked(ToplevelKey),
    CloseAllApps,
    SearchTextChanged(String),
    RunApp { name: String, exec: String },
    PowerOptions { show: bool },
    RunningAppsToggle { show: bool },
    Shutdown(ShutdownState),
    Restart(RestartState),
    ChangeLayer(Layer),
    AppOpening { value: bool },
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum SwipeDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum SwipeState {
    #[default]
    UserSwiping,
    CompletingSwipe,
    Completed,
    CancellingSwipe,
    Cancelled,
}

#[derive(Debug, Clone, Default)]
pub struct Swipe {
    pub dx: i32,
    pub dy: i32,
    pub max_dx: i32,
    pub max_dy: i32,
    pub min_dx: i32,
    pub min_dy: i32,
    pub direction: SwipeDirection,
    pub state: SwipeState,
    pub is_closer: bool,
    pub threshold_dy: i32,
    pub translations: Vec<f64>,
    pub current_translation: usize,
}

impl Hash for Swipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dx.hash(state);
        self.dy.hash(state);
        self.max_dx.hash(state);
        self.max_dy.hash(state);
        self.min_dx.hash(state);
        self.min_dy.hash(state);
        self.direction.hash(state);
        self.state.hash(state);
        self.is_closer.hash(state);
        self.threshold_dy.hash(state);
        self.translations.len().hash(state);
        self.current_translation.hash(state);
    }
}

impl Swipe {
    fn reverse(&self) -> Self {
        let Swipe {
            dx,
            dy,
            max_dx,
            max_dy,
            min_dx,
            min_dy,
            mut direction,
            state,
            mut is_closer,
            threshold_dy,
            translations,
            current_translation,
        } = self.clone();

        match direction {
            SwipeDirection::Up => {
                direction = SwipeDirection::Down;
            }
            SwipeDirection::Down => {
                direction = SwipeDirection::Up;
            }
            SwipeDirection::Left => {
                direction = SwipeDirection::Right;
            }
            SwipeDirection::Right => {
                direction = SwipeDirection::Left;
            }
        };

        if is_closer {
            is_closer = false;
        } else {
            is_closer = true;
        }

        let swipe = Swipe {
            direction,
            state: SwipeState::CompletingSwipe,
            is_closer,
            ..self.clone()
        };

        swipe
    }
}

#[derive(Debug)]
pub struct LauncherState {
    settings: LauncherSettings,
    custom_theme: LauncherTheme,
    battery_level: BatteryLevel,
    wireless_status: WirelessStatus,
    bluetooth_status: BluetoothStatus,
    rotation_status: RotationStatus,
    time: String,
    date: String,
    cpu_usage: VecDeque<u8>,
    uptime: String,
    machine_name: String,
    ip_address: String,
    online: bool,
    used_memory: u64,
    current_screen: Screens,
    swipe: Option<Swipe>,
    active_swipe: Option<Swipe>,
    sound: u8,
    brightness: u8,
    lock_slide: u8,
    running_apps_count: i32,
    app_channel: Option<Sender<AppMessage>>,
    running_apps: Vec<RunningApp>,
    installed_apps: Vec<DesktopEntry>,
    show_power_options: bool,
    show_running_apps: bool,
    shutdown_pressed: bool,
    restart_pressed: bool,
    current_layer: Layer,
    app_opening: bool,
}

#[component(State = "LauncherState")]
#[derive(Debug, Default)]
pub struct Launcher {}
impl Launcher {
    fn handle_on_drag(&mut self, logical_delta: Point) -> Option<mctk_core::component::Message> {
        let dx = logical_delta.x;
        let dy = logical_delta.y;
        println!("dx {:?} dy {:?}", dx, dy);
        if let Some(mut swipe) = self.state_ref().swipe.clone() {
            match swipe.direction {
                SwipeDirection::Up => {
                    if dy > 0. {
                        self.state_mut().swipe = None;
                        self.state_mut().active_swipe = None;
                        return None;
                    }

                    swipe.dx = -dx as i32;
                    swipe.dy = (480. + dy) as i32;
                }
                SwipeDirection::Down => {
                    if dy < 0. {
                        self.state_mut().swipe = None;
                        self.state_mut().active_swipe = None;
                        return None;
                    }

                    swipe.dx = dx as i32;
                    swipe.dy = dy as i32;
                    // if dy as i32 > swipe.threshold_dy
                    //     && self.state_ref().running_apps_count > 0
                    //     && self.state_ref().current_layer != Layer::Top
                    // {
                    //     if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    //         let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Top));
                    //     };
                    // }
                }
                SwipeDirection::Left => {}
                SwipeDirection::Right => {}
            }

            self.state_mut().swipe = Some(swipe);
        };

        None
    }

    pub fn handle_on_drag_end(&mut self) {
        println!(
            "Launcher::handle_on_drag_end() {:?}",
            self.state_ref().swipe.clone()
        );
        if let Some(mut swipe) = self.state_ref().swipe.clone() {
            if swipe.state == SwipeState::UserSwiping {
                let dy = swipe.dy;
                let translations = match swipe.direction {
                    SwipeDirection::Up => {
                        let inc = 1.0 / 12.0;
                        let t = (480 - dy) as f64 / 480.0 + inc;
                        let trans = get_translations(BEZIER_POINTS, t, inc);
                        if trans.len() > 0 {
                            trans
                        } else {
                            vec![480.]
                        }

                        // let mut counter = translate_y.len() - 1;
                        // for (i, &value) in translate_y.iter().rev().enumerate() {
                        //     if value >= dy as f64 {
                        //         counter = i;
                        //     }
                        // }
                        // counter
                    }
                    SwipeDirection::Down => {
                        let inc = 1.0 / 12.0;
                        let t = dy as f64 / 480.0 + inc;
                        let trans = get_translations(BEZIER_POINTS, t, inc);
                        if trans.len() > 0 {
                            trans
                        } else {
                            vec![480.]
                        }

                        // let mut counter = 0;
                        // for (i, &value) in translate_y.iter().enumerate() {
                        //     if dy as f64 >= value {
                        //         counter = i;
                        //     } else {
                        //         break;
                        //     }
                        // }
                        // counter
                    }

                    SwipeDirection::Left => todo!(),
                    SwipeDirection::Right => todo!(),
                };

                println!("translations is {:?}", translations);
                swipe.translations = translations;
                swipe.state = SwipeState::CompletingSwipe;
                self.state_mut().swipe = Some(swipe);
            }
        }
    }

    pub fn is_drag_from_edges(
        &self,
        current_logical_aabb: AABB,
        relative_logical_position: Point,
    ) -> Option<(AABB, AABB, AABB, AABB)> {
        let aabb = current_logical_aabb;
        let pos = relative_logical_position;
        let top_area = aabb.set_scale(aabb.width(), 36.);
        let left_area = aabb.set_scale(20., aabb.height());
        let right_area = aabb
            .set_top_left(460., aabb.pos.y)
            .set_scale(20., aabb.height());
        let bottom_area = aabb
            .set_top_left(aabb.pos.x, 460.)
            .set_scale(aabb.width(), 20.);

        if !(top_area.is_under(pos)
            || left_area.is_under(pos)
            || bottom_area.is_under(pos)
            || right_area.is_under(pos))
        {
            return None;
        }

        Some((top_area, left_area, bottom_area, right_area))
    }

    pub fn handle_on_drag_start(
        &mut self,
        pos: Point,
        edges: (AABB, AABB, AABB, AABB),
        aabb: AABB,
    ) {
        let (top_area, left_area, bottom_area, right_area) = edges;

        println!(
            "under {:?} {:?} {:?} {:?}",
            top_area.is_under(pos),
            left_area.is_under(pos),
            bottom_area.is_under(pos),
            right_area.is_under(pos),
        );
        let mut swipe = None;

        if top_area.is_under(pos) {
            swipe = Some(Swipe {
                max_dy: aabb.height() as i32,
                min_dy: 0,
                threshold_dy: 88,
                direction: SwipeDirection::Down,
                ..Default::default()
            });
        } else if bottom_area.is_under(pos) {
            swipe = Some(Swipe {
                dy: 480,
                max_dy: aabb.height() as i32,
                min_dy: 0,
                direction: SwipeDirection::Up,
                ..Default::default()
            })
        }
        // else if left_area.is_under(pos) {
        //     swipe = Some(Swipe {
        //         max_dx: aabb.width() as i32,
        //         min_dx: 0,
        //         direction: SwipeDirection::Right,
        //         ..Default::default()
        //     })
        // } else if right_area.is_under(pos) {
        //     swipe = Some(Swipe {
        //         max_dx: aabb.width() as i32,
        //         min_dx: 0,
        //         direction: SwipeDirection::Left,
        //         ..Default::default()
        //     })
        // }

        self.state_mut().swipe = swipe;
    }
}

#[state_component_impl(LauncherState)]
impl Component for Launcher {
    fn on_tick(&mut self, _event: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
        if self.state.is_none() {
            return;
        }

        // println!("on_tick swipe {:?}", self.state_ref().swipe.clone());
        if let Some(mut swipe) = self.state_ref().swipe.clone() {
            let Swipe {
                dx,
                mut dy,
                max_dx,
                max_dy,
                min_dx,
                min_dy,
                direction,
                state,
                is_closer,
                threshold_dy,
                translations,
                current_translation,
            } = swipe.clone();

            // println!("Launcher::on_tick() dy {:?} {:?}", dy, state);
            if state == SwipeState::Completed || state == SwipeState::Cancelled {
                if is_closer {
                    self.state_mut().swipe = None;
                }
                return;
            }

            if state == SwipeState::CancellingSwipe {
                if direction == SwipeDirection::Down {
                    if dy < threshold_dy {
                        self.state_mut().swipe = Some(swipe.reverse());
                        return;
                    }
                } else if direction == SwipeDirection::Up {
                    if (max_dy - dy) < threshold_dy {
                        self.state_mut().swipe = Some(swipe.reverse());
                        return;
                    }
                }
            }

            if state == SwipeState::CompletingSwipe {
                if direction == SwipeDirection::Down {
                    if dy >= max_dy {
                        swipe.state = SwipeState::Completed;
                        // return;
                    }

                    swipe.dy = (translations[current_translation] as i32)
                        .max(min_dy)
                        .min(max_dy);
                    println!("swipe.dy {:?}", swipe.dy);
                }
                if direction == SwipeDirection::Up {
                    println!("dy {:?} min_dy {:?}", dy, min_dy);
                    if dy <= min_dy {
                        swipe.state = SwipeState::Completed;
                        // if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        //     let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Bottom));
                        // }
                        // return;
                    }
                    swipe.dy = (480 - translations[current_translation] as i32)
                        .max(min_dy)
                        .min(max_dy);
                    // swipe.dy = (dy - 8).max(min_dy).min(max_dy);
                    println!("swipe.dy {:?}", swipe.dy);
                }
                swipe.current_translation = max(
                    0,
                    min(
                        translations.len().saturating_sub(1),
                        current_translation + 1,
                    ),
                );
            }
            self.state_mut().swipe = Some(swipe);
            println!("updated swipe");
        }
    }

    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LauncherSettings::default(),
        };

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LauncherTheme::default(),
        };
        self.state = Some(LauncherState {
            settings,
            custom_theme,
            battery_level: BatteryLevel::default(),
            wireless_status: WirelessStatus::default(),
            bluetooth_status: BluetoothStatus::default(),
            rotation_status: RotationStatus::default(),
            time: String::from(""),
            date: String::from(""),
            cpu_usage: VecDeque::new(),
            uptime: String::new(),
            machine_name: String::new(),
            ip_address: String::new(),
            online: false,
            used_memory: 0,
            current_screen: Screens::Home,
            swipe: None,
            active_swipe: None,
            sound: 0,
            brightness: 0,
            running_apps_count: 0,
            app_channel: None,
            lock_slide: 0,
            running_apps: vec![],
            installed_apps: vec![],
            show_power_options: false,
            show_running_apps: false,
            shutdown_pressed: false,
            restart_pressed: false,
            current_layer: Layer::Bottom,
            app_opening: false,
        });
    }

    fn view(&self) -> Option<Node> {
        let cpu_usage = self.state_ref().cpu_usage.clone();
        let uptime = self.state_ref().uptime.clone();
        let machine_name = self.state_ref().machine_name.clone();
        let ip_address = self.state_ref().ip_address.clone();
        let online = self.state_ref().online.clone();
        let used_memory = self.state_ref().used_memory;
        let time = self.state_ref().time.clone();
        let date = self.state_ref().date.clone();
        let battery_level = self.state_ref().battery_level.clone();
        let wireless_status = self.state_ref().wireless_status.clone();
        let bluetooth_status = self.state_ref().bluetooth_status.clone();
        let rotation_status = self.state_ref().rotation_status.clone();
        let settings = self.state_ref().settings.clone();
        let current_screen = self.state_ref().current_screen.clone();
        let swipe = self.state_ref().swipe.clone();
        let active_swipe = self.state_ref().active_swipe.clone();
        // println!("swipe is {:?} {:?}", swipe, active_swipe);
        let sound = self.state_ref().sound;
        let brightness = self.state_ref().brightness;
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let running_apps = self.state_ref().running_apps.clone();
        let installed_apps = self.state_ref().installed_apps.clone();
        let show_power_options = self.state_ref().show_power_options;
        let show_running_apps = self.state_ref().show_running_apps;
        let shutdown_pressed = self.state_ref().shutdown_pressed;
        let restart_pressed = self.state_ref().restart_pressed;
        let app_opening = self.state_ref().app_opening;

        let mut start_node = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.64)),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                direction: Direction::Column,
                // padding: [22]
            ]
        );

        if on_top_of_other_apps || !(current_screen == Screens::Home) || show_running_apps {
            start_node = start_node.push(node!(
                StatusBar {
                    battery_level,
                    wireless_status,
                    bluetooth_status,
                    current_time: time.clone()
                },
                lay![ size: [Auto, 36] ]
            ));
        }

        // start_node = start_node.push(node!(
        //     AppSwitcher { running_apps },
        //     lay![size_pct: [100, Auto],]
        // ));

        if show_running_apps {
            start_node = start_node.push(node!(
                AppSwitcher { running_apps },
                lay![
                    size_pct: [100],
                    position_type: Absolute,
                    position: [36., 0., 0., 0.],
                ]
            ));
        }

        let mut down_swipe = 0;
        let mut up_swipe = 480;
        if let Some(swipe) = swipe.clone() {
            if (swipe.direction == SwipeDirection::Down && !swipe.is_closer)
                || (swipe.direction == SwipeDirection::Up && swipe.is_closer)
            {
                down_swipe = swipe.dy;
            } else if (swipe.direction == SwipeDirection::Up && !swipe.is_closer)
                || (swipe.direction == SwipeDirection::Down && swipe.is_closer)
            {
                up_swipe = swipe.dy;
            }
        } else if let Some(swipe) = active_swipe.clone() {
            if (swipe.direction == SwipeDirection::Down && !swipe.is_closer)
                || (swipe.direction == SwipeDirection::Up && swipe.is_closer)
            {
                down_swipe = swipe.dy;
            } else if (swipe.direction == SwipeDirection::Up && !swipe.is_closer)
                || (swipe.direction == SwipeDirection::Down && swipe.is_closer)
            {
                up_swipe = swipe.dy;
            }
        }

        if down_swipe.abs() > 20 {
            start_node = start_node.push(node!(
                SettingsPanel {
                    swipe: down_swipe,
                    sound,
                    brightness,
                    battery_level,
                    wireless_status,
                    bluetooth_status,
                    rotation_status
                },
                lay![
                    size_pct: [100],
                    position_type: Absolute,
                    position: [0., 0., 0., 0.],
                ]
            ));
        }

        if (up_swipe.abs()) < 460 {
            // println!("up_swipe {:?} ", up_swipe);
            start_node = start_node.push(node!(
                AppDrawer::new(installed_apps, up_swipe, app_opening),
                lay![
                    size_pct: [100],
                    position_type: Absolute,
                    position: [0., 0., 0., 0.],
                ]
            ));
        }

        if show_power_options {
            start_node = start_node.push(node!(
                PowerOptions {
                    shutdown_pressed,
                    restart_pressed
                },
                lay![
                    size_pct: [100],
                    position_type: Absolute,
                    position: [0., 0., 0., 0.],
                ]
            ));
        }

        if !on_top_of_other_apps && !show_running_apps {
            start_node = start_node.push(node!(
                HomeUi {
                    settings,
                    battery_level,
                    wireless_status,
                    bluetooth_status,
                    time,
                    date,
                    cpu_usage,
                    uptime,
                    machine_name,
                    ip_address,
                    online,
                    used_memory,
                    is_lock_screen: false,
                    disable_activity: (swipe.is_some() || active_swipe.is_some() || app_opening)
                },
                lay![size_pct: [100, Auto],]
            ));
        }

        Some(start_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message);
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::AppClicked { app_id } => {
                    println!("app clicked {:?}", app_id);
                    let apps = self.state_ref().settings.modules.apps.clone();
                    let app = apps.into_iter().find(|app| app.app_id == *app_id).unwrap();
                    let run_command = app.run_command;
                    if !run_command.is_empty() {
                        let command = run_command[0].clone();
                        let args: Vec<String> = run_command.clone()[1..].to_vec();
                        self.state_mut().app_opening = true;
                        let _ = spawn_command(command, args);
                    }
                }
                Message::AppListAppClicked { app } => {
                    let mut args: Vec<String> = vec!["-c".to_string()];
                    args.push(app.exec.clone());
                    self.state_mut().app_opening = true;
                    let _ = spawn_command("sh".to_string(), args);
                }
                Message::Clock { time, date } => {
                    self.state_mut().time = time.clone();
                    self.state_mut().date = date.clone().to_uppercase();
                }
                Message::Wireless { status } => {
                    println!("Message::Wireless {:?}", status);
                    self.state_mut().wireless_status = status.clone();
                }
                Message::Bluetooth { status } => {
                    self.state_mut().bluetooth_status = status.clone();
                }
                Message::Battery { level, status } => {
                    let battery_level = get_formatted_battery_level(level, status);
                    self.state_mut().battery_level = battery_level;
                }
                Message::SettingClicked(settings_name) => {
                    println!("setting clicked: {:?}", settings_name);
                    match settings_name {
                        SettingNames::Wireless => {
                            let wireless_status = self.state_ref().wireless_status.clone();
                            let value = match wireless_status {
                                WirelessStatus::Off => true,
                                WirelessStatus::On => false,
                                WirelessStatus::Connected(_) => false,
                                WirelessStatus::NotFound => false,
                            };
                            if let Some(app_channel) = self.state_ref().app_channel.clone() {
                                if value == true {
                                    self.state_mut().wireless_status = WirelessStatus::On;
                                } else {
                                    self.state_mut().wireless_status = WirelessStatus::Off;
                                }
                                let _ = app_channel.send(AppMessage::Wireless {
                                    message: WirelessMessage::Toggle { value: Some(value) },
                                });
                            }
                        }
                        SettingNames::Bluetooth => {
                            let bluetooth_status = self.state_ref().bluetooth_status.clone();
                            let value = match bluetooth_status {
                                BluetoothStatus::Off => true,
                                BluetoothStatus::On => false,
                                BluetoothStatus::Connected => false,
                                BluetoothStatus::NotFound => false,
                            };
                            if let Some(app_channel) = self.state_ref().app_channel.clone() {
                                if value == true {
                                    self.state_mut().bluetooth_status = BluetoothStatus::On;
                                } else {
                                    self.state_mut().bluetooth_status = BluetoothStatus::Off;
                                }
                                let _ = app_channel.send(AppMessage::Bluetooth {
                                    message: BluetoothMessage::Toggle { value: Some(value) },
                                });
                            }
                        }
                        SettingNames::Rotation => {
                            let rotation_status = self.state_ref().rotation_status.clone();
                            if rotation_status == RotationStatus::Portrait {
                                self.state_mut().rotation_status = RotationStatus::Landscape;
                            } else if rotation_status == RotationStatus::Landscape {
                                self.state_mut().rotation_status = RotationStatus::Portrait;
                            }
                        }
                        SettingNames::Terminal => {
                            let run_command = self
                                .state_ref()
                                .settings
                                .modules
                                .terminal
                                .run_command
                                .clone();
                            println!("run_command {:?}", run_command);
                            if !run_command.is_empty() {
                                let command = run_command[0].clone();
                                let args: Vec<String> = run_command.clone()[1..].to_vec();
                                println!("command {:?} args {:?}", command, args);
                                let _ = spawn_command(command, args);

                                let inc = 1.0 / 12.0;
                                let translations = get_translations(BEZIER_POINTS, 0., inc);
                                let swipe = Swipe {
                                    dy: 480 as i32,
                                    min_dy: 0,
                                    max_dy: 480,
                                    threshold_dy: 0,
                                    direction: SwipeDirection::Up,
                                    state: SwipeState::CompletingSwipe,
                                    is_closer: true,
                                    translations,
                                    ..Default::default()
                                };

                                self.state_mut().swipe = Some(swipe);
                            }
                        }
                        SettingNames::Power => {
                            self.state_mut().show_power_options = true;
                            self.state_mut().swipe = None;
                        }
                    }
                }
                Message::SliderChanged(settings_name) => match settings_name {
                    SliderSettingsNames::Brightness { value } => {
                        // self.state_mut().brightness = *value;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::Brightness {
                                message: BrightnessMessage::Change {
                                    value: *value as u8,
                                },
                            });
                        }
                    }
                    SliderSettingsNames::Sound { value } => {
                        // self.state_mut().sound = *value;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::Sound {
                                message: SoundMessage::Change {
                                    value: *value as u8,
                                },
                            });
                        }
                    }
                    SliderSettingsNames::Lock { value } => {
                        self.state_mut().lock_slide = *value;
                    }
                },
                Message::CPUUsage { usage } => {
                    let mut usages = self.state_ref().cpu_usage.clone();
                    usages.push_front(*usage as u8);
                    if usages.len() as u8 > GRID_SIZE.0 + 1 {
                        usages.pop_back();
                    }
                    self.state_mut().cpu_usage = usages.clone();
                }
                Message::Uptime { uptime } => {
                    self.state_mut().uptime = uptime.clone();
                }
                Message::MachineName { name } => {
                    self.state_mut().machine_name = name.clone();
                }
                Message::IpAddress { address } => {
                    self.state_mut().ip_address = address.clone();
                }
                Message::Net { online } => {
                    self.state_mut().online = online.clone();
                }
                Message::Memory { total, used } => {
                    self.state_mut().used_memory = ((*used as f64 / *total as f64) * 100.) as u64;
                }
                Message::Swipe { swipe } => {
                    // Handle swipe gestures
                    println!("Swipe detected: {:?}", swipe);
                    self.state_mut().swipe = Some(swipe.clone());
                    // self.state_mut().swipe_gesture = Some(direction.clone());
                }
                Message::Sound { value } => {
                    self.state_mut().sound = *value;
                }
                Message::Brightness { value } => {
                    self.state_mut().brightness = *value;
                }
                Message::SwipeEnd => {
                    if let Some(mut swipe) = self.state_ref().swipe.clone() {
                        let dy = swipe.dy;
                        let translations = match swipe.direction {
                            SwipeDirection::Up => {
                                let inc = 1.0 / 12.0;
                                let t = (480 - dy) as f64 / 480.0 + inc;
                                let trans = get_translations(BEZIER_POINTS, t, inc);
                                if trans.len() > 0 {
                                    trans
                                } else {
                                    vec![480.]
                                }
                                // let mut counter = translate_y.len() - 1;
                                // for (i, &value) in translate_y.iter().rev().enumerate() {
                                //     if value >= dy as f64 {
                                //         counter = i;
                                //     }
                                // }
                                // counter
                            }
                            SwipeDirection::Down => {
                                let inc = 1.0 / 12.0;
                                let t = dy as f64 / 480.0 + inc;
                                let trans = get_translations(BEZIER_POINTS, t, inc);
                                if trans.len() > 0 {
                                    trans
                                } else {
                                    vec![480.]
                                }
                                // let mut counter = 0;
                                // for (i, &value) in translate_y.iter().enumerate() {
                                //     if dy as f64 >= value {
                                //         counter = i;
                                //     } else {
                                //         break;
                                //     }
                                // }
                                // counter
                            }
                            SwipeDirection::Left => todo!(),
                            SwipeDirection::Right => todo!(),
                        };
                        println!("translations is {:?}", translations);
                        swipe.translations = translations;
                        swipe.state = SwipeState::CompletingSwipe;
                        self.state_mut().swipe = Some(swipe);
                    }
                }
                Message::RunningApps { count } => {
                    self.state_mut().running_apps_count = *count;
                }
                Message::AppsUpdated { apps } => {
                    println!("apps updated are {:?}", apps.len());
                    let settings = self.state_ref().settings.clone();
                    let app_id = settings.app.id.clone().unwrap_or_default();
                    self.state_mut().running_apps = apps
                        .iter()
                        .filter(|app| {
                            app.app_id != app_id
                                && settings
                                    .modules
                                    .running_apps
                                    .exclude
                                    .clone()
                                    .into_iter()
                                    .any(|ea| ea != app.app_id)
                        })
                        .map(|app| RunningApp::new(AppDetails { ..app.clone() }))
                        .collect();
                    println!(
                        "running apps state updated {:?}",
                        self.state_ref().running_apps.len()
                    );
                }
                Message::AppInstanceClicked(instance) => {
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::AppInstanceClicked(*instance));
                        let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Bottom));
                        self.state_mut().show_running_apps = false;
                        // process::exit(0)
                    };
                }
                Message::AppInstanceCloseClicked(instance) => {
                    let running_apps = self.state_ref().running_apps.clone();
                    let filtered_running_apps: Vec<RunningApp> = running_apps
                        .clone()
                        .into_iter()
                        .filter(|app| {
                            app.app_details.instances.get(0).unwrap().instance_key != *instance
                        })
                        .collect();
                    if filtered_running_apps.len() == 0 {
                        self.state_mut().show_running_apps = false;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Bottom));
                        };
                    }
                    self.state_mut().running_apps = filtered_running_apps;
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        if let Some(app) = running_apps.into_iter().find(|app| {
                            app.app_details.instances.get(0).unwrap().instance_key == *instance
                        }) {
                            for instance in app.app_details.instances {
                                let _ = app_channel.send(AppMessage::AppInstanceCloseClicked(
                                    instance.instance_key,
                                ));
                            }
                        };
                    };
                }
                Message::CloseAllApps => {
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::CloseAllApps);
                    };
                }
                Message::PowerOptions { show } => {
                    self.state_mut().show_power_options = *show;
                    if !show {
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Bottom));
                        };
                    }
                }
                Message::RunningAppsToggle { show } => {
                    self.state_mut().show_running_apps = *show;
                    if !show {
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::ChangeLayer(Layer::Bottom));
                        };
                    }
                }
                Message::Shutdown(state) => match state {
                    ShutdownState::Pressed => {
                        self.state_mut().shutdown_pressed = true;
                    }
                    ShutdownState::Released => {
                        self.state_mut().shutdown_pressed = false;
                    }
                    ShutdownState::Clicked => {
                        if let Some(app_channel) = &self.state_ref().app_channel {
                            let _ = app_channel.send(AppMessage::ShutDown);
                        }
                    }
                },
                Message::Restart(state) => match state {
                    RestartState::Pressed => {
                        self.state_mut().restart_pressed = true;
                    }
                    RestartState::Released => {
                        self.state_mut().restart_pressed = false;
                    }
                    RestartState::Clicked => {
                        if let Some(app_channel) = &self.state_ref().app_channel {
                            let _ = app_channel.send(AppMessage::Restart);
                        }
                    }
                },
                Message::ChangeLayer(layer) => {
                    self.state_mut().current_layer = *layer;
                }
                Message::AppOpening { value } => {
                    self.state_mut().app_opening = *value;
                }
                _ => (),
            }
        }
        vec![]
    }

    fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
        if self.state_ref().running_apps_count > 0 {
            return None;
        }

        let mut rs = vec![];
        if self
            .state_ref()
            .settings
            .modules
            .background
            .icon
            .default
            .len()
            > 0
        {
            let width = context.aabb.width();
            let height = context.aabb.height();
            let AABB { pos, .. } = context.aabb;

            let image = Image::new(pos, Scale { width, height }, "background")
                .composite_operation(CompositeOperation::DestinationOver);

            rs.push(Renderable::Image(image));
        } else {
            let rect_instance = InstanceBuilder::default()
                .pos(Pos {
                    x: context.aabb.pos.x,
                    y: context.aabb.pos.y,
                    z: 0.1,
                })
                .scale(context.aabb.size())
                .color(Color::BLACK)
                .build()
                .unwrap();

            rs.push(Renderable::Rect(Rect::from_instance_data(rect_instance)))
        }

        Some(rs)
    }

    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().swipe.hash(hasher);
        self.state_ref().active_swipe.hash(hasher);
        self.state_ref().running_apps_count.hash(hasher);
        self.state_ref().show_power_options.hash(hasher);
        self.state_ref().show_running_apps.hash(hasher);
        // println!("render hash is {:?}", hasher.finish());
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }

        if self.state_ref().swipe.is_some() {
            println!("Swipe already exists");
            return;
        }

        println!(
            "Launcher::on_drag_start() {:?}",
            event.physical_mouse_position()
        );

        let aabb = event.current_logical_aabb();
        let pos = event.physical_mouse_position();

        if let Some(edges) = self.is_drag_from_edges(aabb, pos) {
            event.stop_bubbling();
            self.handle_on_drag_start(pos, edges, aabb);
        };
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }

        println!(
            "Launcher::on_touch_drag_start() {:?}",
            event.physical_touch_position()
        );
        if self.state_ref().swipe.is_some() {
            println!("Swipe already exists");
            return;
        }

        let aabb = event.current_logical_aabb();
        let pos = event.physical_touch_position();

        if let Some(edges) = self.is_drag_from_edges(aabb, pos) {
            event.stop_bubbling();
            self.handle_on_drag_start(pos, edges, aabb);
        };
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }
        let logical_delta = event.bounded_logical_delta();
        println!("Launcher::on_drag() {:?}", logical_delta);
        if let Some(msg) = self.handle_on_drag(logical_delta) {
            self.update(msg);
        }
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }

        let logical_delta = event.bounded_logical_delta();
        println!("Launcher::on_touch_drag() {:?}", logical_delta);
        if let Some(msg) = self.handle_on_drag(logical_delta) {
            self.update(msg);
        }
    }

    fn on_drag_end(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }
        self.handle_on_drag_end();
    }

    fn on_touch_drag_end(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;
        let app_opening = self.state_ref().app_opening;

        if app_opening {
            println!("not dragging as some app is launching");
            return;
        }

        if on_top_of_other_apps {
            println!("not dragging as other apps are running");
            return;
        }
        println!(
            "Launcher::on_touch_drag_end() {:?}",
            event.physical_touch_position()
        );
        self.handle_on_drag_end();
    }
}

impl RootComponent<AppParams> for Launcher {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        let app_channel = app_params.app_channel.clone();
        let installed_apps = app_params.installed_apps.clone();
        self.state_mut().app_channel = app_channel;
        if let Some(apps) = installed_apps {
            self.state_mut().installed_apps = apps
        }
    }
}

pub fn get_translations(bezier_points: [f64; 4], start_time: f64, increment: f64) -> Vec<f64> {
    let mut time = start_time;
    let mut translations = Vec::new();
    while time <= 1.0 {
        if (time + increment) > 1.0 {
            time = 1.0;
        }
        translations.push(cubic_bezier(&bezier_points, time));
        time += increment;
    }
    translations
}
