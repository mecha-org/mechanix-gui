use crate::modules::cpu::component::GRID_SIZE;
use crate::modules::settings_panel::rotation::component::RotationStatus;
use crate::pages::home_ui::HomeUi;
use crate::pages::settings_panel::SettingsPanel;
use crate::pages::status_bar::StatusBar;
use crate::settings::{self, LauncherSettings};
use crate::theme::{self, LauncherTheme};
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use crate::utils::get_formatted_battery_level;
use crate::{
    AppMessage, AppParams, BluetoothMessage, BrightnessMessage, SoundMessage, WirelessMessage,
};
use command::spawn_command;
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::{Image, Renderable};
use mctk_core::{component, msg, Color, Point, Scale, AABB};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};
use std::any::Any;
use std::collections::VecDeque;
use std::hash::Hash;
use upower::BatteryStatus;

#[derive(Debug, Clone, Hash)]
pub enum SwipeGestures {
    Down(i32),
    Up(i32),
    Right(i32),
    Left(i32),
}

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
    Settings,
    Power,
}
#[derive(Debug, Clone)]
pub enum SliderSettingsNames {
    Brightness { value: u8 },
    Sound { value: u8 },
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
    SwipeEnd,
    Sound { value: u8 },
    Brightness { value: u8 },
    RunningApps { count: i32 },
    Unlock,
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
}

#[derive(Debug, Clone, Default, Hash)]
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
}

#[derive(Debug, Default)]
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
    swipe_gesture: Option<SwipeGestures>,
    swipe: Option<Swipe>,
    sound: u8,
    brightness: u8,
    running_apps_count: i32,
    app_channel: Option<Sender<AppMessage>>,
}

#[component(State = "LauncherState")]
#[derive(Debug, Default)]
pub struct Launcher {}
impl Launcher {
    fn handle_on_drag(&mut self, logical_delta: Point) -> Option<mctk_core::component::Message> {
        let dx = logical_delta.x;
        let dy = logical_delta.y;
        // println!("dx {:?} dy {:?}", dx, dy);
        if let Some(mut swipe) = self.state_ref().swipe.clone() {
            swipe.dx = dx as i32;
            swipe.dy = dy as i32;
            self.state_mut().swipe = Some(swipe);
        };

        // let min_drag = 10.;
        // if dx.abs() <= min_drag && dy.abs() <= min_drag {
        //     return None;
        // }

        // let swipe_gesture = self.state_ref().swipe_gesture.clone();
        // if let Some(swipe_gesture) = swipe_gesture {
        //     match swipe_gesture {
        //         SwipeGestures::Down(_) => {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Down(dy.abs() as i32)
        //             }));
        //         }
        //         SwipeGestures::Up(_) => {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Down(dy.abs() as i32)
        //             }));
        //         }
        //         SwipeGestures::Right(_) => {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Right(dx.abs() as i32)
        //             }));
        //         }
        //         SwipeGestures::Left(_) => {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Left(dx.abs() as i32)
        //             }));
        //         }
        //     }
        // };

        // if dx.abs() > min_drag || dy.abs() > min_drag {
        //     if dx > dy {
        //         if logical_delta.x > 0. {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Right(dx.abs() as i32)
        //             }));
        //         } else {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Left(dx.abs() as i32)
        //             }));
        //         }
        //     } else {
        //         if logical_delta.y > 0. {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Down(dy.abs() as i32)
        //             }));
        //         } else {
        //             return Some(msg!(Message::Swipe {
        //                 direction: SwipeGestures::Up(dy.abs() as i32)
        //             }));
        //         }
        //     };
        // }

        None
    }

    pub fn handle_on_drag_end(&mut self) {
        if let Some(mut swipe) = self.state_ref().swipe.clone() {
            if swipe.state == SwipeState::UserSwiping {
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
                direction: SwipeDirection::Down,
                ..Default::default()
            });
        }
        // else if bottom_area.is_under(pos) {
        //     swipe = Some(Swipe {
        //         max_dy: aabb.height() as i32,
        //         min_dy: 0,
        //         direction: SwipeDirection::Up,
        //         ..Default::default()
        //     })
        // } else if left_area.is_under(pos) {
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
        if let Some(app_channel) = self.state_ref().app_channel.clone() {
            let _ = app_channel.send(AppMessage::RunOnTop);
        };
    }
}

#[state_component_impl(LauncherState)]
impl Component for Launcher {
    fn on_tick(&mut self, _event: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
        if self.state.is_none() {
            return;
        }

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
            } = swipe.clone();

            println!("Launcher::on_tick() dy {:?} {:?}", dy, state);
            if state == SwipeState::Completed {
                if is_closer {
                    self.state_mut().swipe = None;
                }
                return;
            }

            if state == SwipeState::CompletingSwipe {
                if direction == SwipeDirection::Down {
                    if dy >= max_dy {
                        swipe.state = SwipeState::Completed;
                        // return;
                    }

                    swipe.dy = (dy + 30).max(min_dy).min(max_dy);
                }
                if direction == SwipeDirection::Up {
                    if dy <= min_dy {
                        swipe.state = SwipeState::Completed;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::RunOnBottom);
                        }
                        // return;
                    }

                    swipe.dy = (dy - 30).max(min_dy).min(max_dy);
                }
            }
            self.state_mut().swipe = Some(swipe);

            // if self.state.is_some() {
            //     if let Some(ongoing) = self.state_ref().swipe_gesture.clone() {
            //         match ongoing {
            //             SwipeGestures::Down(y) => {
            //                 if y > 100 {
            //                     self.state_mut().current_screen = Screens::AppDrawer;
            //                 }
            //             }
            //             SwipeGestures::Up(y) => {
            //                 if y > 100 {
            //                     self.state_mut().current_screen = Screens::Home;
            //                 }
            //             }
            //             SwipeGestures::Right(_) => {}
            //             SwipeGestures::Left(_) => {}
            //         }
            //     };
            // }
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
            swipe_gesture: None,
            swipe: None,
            sound: 0,
            brightness: 0,
            running_apps_count: 0,
            app_channel: None,
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
        let swipe_gesture = self.state_ref().swipe_gesture.clone();
        let swipe = self.state_ref().swipe.clone();
        println!("swipe is {:?}", swipe);
        let sound = self.state_ref().sound;
        let brightness = self.state_ref().brightness;
        let on_top_of_other_apps = self.state_ref().running_apps_count > 0;

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

        if on_top_of_other_apps || !(current_screen == Screens::Home) {
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

        let mut down_swipe = 0;
        if let Some(swipe) = swipe {
            if (swipe.direction == SwipeDirection::Down && !swipe.is_closer)
                || (swipe.direction == SwipeDirection::Up && swipe.is_closer)
            {
                down_swipe = swipe.dy;
            }
        }

        // if let Some(swipe) = self.state_ref().swipe_gesture.clone() {
        //     match swipe {
        //         SwipeGestures::Down(s) => {
        //             down_swipe = s;
        //         }
        //         SwipeGestures::Up(_) => {}
        //         SwipeGestures::Right(val) => {
        //             // println!("update val {:?}", -480 + val);
        //             // start_node = start_node.push(
        //             //     node!(
        //             //         Div::new().bg(Color::BLUE).swipe(-480 + val),
        //             //         lay![
        //             //             position_type: Absolute,
        //             //             size: [480, 480]
        //             //             position: [Auto, -480 + val, Auto, Auto],
        //             //             z_index_increment: 1000.
        //             //         ]
        //             //     )
        //             //     .push(node!(
        //             //         Text::new(txt!("hello"))
        //             //             .style("color", Color::rgb(230., 230., 230.))
        //             //             .style("size", 72.0)
        //             //             .style("font", "SpaceMono-Bold")
        //             //             .style("font_weight", FontWeight::Bold),
        //             //         lay![]
        //             //     ))
        //             //     .key(val as u64),
        //             // );
        //         }
        //         SwipeGestures::Left(_) => {}
        //     }
        // }

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
                    position: [if on_top_of_other_apps { 36. } else  { 0. }, 0., 0., 0.],
                ]
            ));
        }

        if !on_top_of_other_apps {
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
                    swipe_gesture,
                    is_lock_screen: false
                },
                lay![size_pct: [100, Auto],]
            ));
        }

        Some(start_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message);
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
                        let _ = spawn_command(command, args);
                    }
                }
                Message::Clock { time, date } => {
                    self.state_mut().time = time.clone();
                    self.state_mut().date = date.clone().to_uppercase();
                }
                Message::Wireless { status } => {
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
                        SettingNames::Settings => {
                            let run_command = self
                                .state_ref()
                                .settings
                                .modules
                                .settings
                                .run_command
                                .clone();
                            println!("run_command {:?}", run_command);
                            if !run_command.is_empty() {
                                let command = run_command[0].clone();
                                let args: Vec<String> = run_command.clone()[1..].to_vec();
                                println!("command {:?} args {:?}", command, args);
                                let _ = spawn_command(command, args);
                            }
                        }
                        SettingNames::Power => {}
                    }
                }
                Message::SliderChanged(settings_name) => match settings_name {
                    SliderSettingsNames::Brightness { value } => {
                        self.state_mut().brightness = *value;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::Brightness {
                                message: BrightnessMessage::Change {
                                    value: *value as u8,
                                },
                            });
                        }
                    }
                    SliderSettingsNames::Sound { value } => {
                        self.state_mut().sound = *value;
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            let _ = app_channel.send(AppMessage::Sound {
                                message: SoundMessage::Change {
                                    value: *value as u8,
                                },
                            });
                        }
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
                        swipe.state = SwipeState::CompletingSwipe;
                        self.state_mut().swipe = Some(swipe);
                    }
                }
                Message::RunningApps { count } => {
                    self.state_mut().running_apps_count = *count;
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

        let width = context.aabb.width();
        let height = context.aabb.height();
        let AABB { pos, .. } = context.aabb;
        let mut rs = vec![];

        let image = Image::new(pos, Scale { width, height }, "background")
            .composite_operation(CompositeOperation::DestinationOver);

        rs.push(Renderable::Image(image));

        Some(rs)
    }

    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().swipe.hash(hasher);
        self.state_ref().running_apps_count.hash(hasher);
        // println!("render hash is {:?}", hasher.finish());
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        if self.state_ref().swipe.is_some() {
            println!("Swipe already exists");
            return;
        }

        println!(
            "Launcher::on_drag_start() {:?}",
            event.relative_logical_position()
        );

        let aabb = event.current_logical_aabb();
        let pos = event.relative_logical_position();

        if let Some(edges) = self.is_drag_from_edges(aabb, pos) {
            event.stop_bubbling();
            self.handle_on_drag_start(pos, edges, aabb);
        };
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        let logical_delta = event.bounded_logical_delta();
        println!("Launcher::on_drag() {:?}", logical_delta);
        if let Some(msg) = self.handle_on_drag(logical_delta) {
            self.update(msg);
        }
    }

    fn on_drag_end(&mut self, _event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        self.handle_on_drag_end();
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        if self.state_ref().swipe.is_some() {
            println!("Swipe already exists");
            return;
        }

        println!(
            "Launcher::on_drag_start() {:?}",
            event.relative_logical_position_touch()
        );

        let aabb = event.current_logical_aabb();
        let pos = event.relative_logical_position_touch();

        if let Some(edges) = self.is_drag_from_edges(aabb, pos) {
            event.stop_bubbling();
            self.handle_on_drag_start(pos, edges, aabb);
        };
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        let logical_delta = event.bounded_logical_delta();
        println!("Launcher::on_touch_drag() {:?}", logical_delta);
        if let Some(msg) = self.handle_on_drag(logical_delta) {
            self.update(msg);
        }
    }

    fn on_touch_drag_end(
        &mut self,
        _event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        self.handle_on_drag_end();
    }
}

impl RootComponent<AppParams> for Launcher {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        let app_channel = app_params.app_channel.clone();
        self.state_mut().app_channel = app_channel;
    }
}
