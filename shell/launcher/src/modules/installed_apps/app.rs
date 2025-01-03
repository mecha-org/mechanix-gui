use std::{hash::Hash, time::Instant};

use desktop_entries::DesktopEntry;
use mctk_core::component;
use mctk_core::{
    component::{Component, Message},
    event::{Click, Event},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct, state_component_impl,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconType, Image, Svg, Text},
    Color, Node,
};

use super::component::AppListMessage;

#[derive(Debug, Clone, Copy)]
pub enum AppMsg {
    Tick,
}

#[derive(Debug, Default)]
struct AppState {
    pressed: bool,
    pressed_at: Option<Instant>,
}

#[component(State = "AppState", Internal)]
pub struct App {
    pub app: DesktopEntry,
    pub disabled: bool,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("App").field("name", &self.app.name).finish()
    }
}

impl App {
    pub fn new(app: DesktopEntry, disabled: bool) -> Self {
        Self {
            app,
            disabled,
            state: Some(AppState::default()),
            dirty: false,
        }
    }
}

#[state_component_impl(AppState)]
impl Component for App {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        if self.state.is_some() {
            self.state_ref().pressed.hash(hasher);
            self.state_ref().pressed_at.hash(hasher);
        }
    }

    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.app.name.hash(hasher);
    }

    fn on_tick(&mut self, event: &mut Event<mctk_core::event::Tick>) {
        if let Some(pressed_at) = self.state_ref().pressed_at {
            let pressed = self.state_ref().pressed;
            if pressed {
                if pressed_at.elapsed().as_secs_f64() > 1. {
                    self.state_mut().pressed = false;
                    self.state_mut().pressed_at = None;
                    event.emit(msg!(AppListMessage::AppLongClicked {
                        app: self.app.clone()
                    }))
                }
            } else {
                self.state_mut().pressed_at = None;
                event.emit(msg!(AppListMessage::AppClicked {
                    app: self.app.clone()
                }))
            }
        }
        event.stop_bubbling();
    }

    fn on_mouse_down(&mut self, _event: &mut Event<mctk_core::event::MouseDown>) {
        if self.disabled {
            return;
        }

        self.state_mut().pressed = true;
        self.state_mut().pressed_at = Some(Instant::now());
    }

    fn on_mouse_up(&mut self, _event: &mut Event<mctk_core::event::MouseUp>) {
        if self.disabled {
            return;
        }

        self.state_mut().pressed = false;
    }

    fn on_drag_start(&mut self, _event: &mut Event<mctk_core::event::DragStart>) {
        self.state_mut().pressed_at = None;
        self.state_mut().pressed = false;
    }

    fn on_touch_down(&mut self, _event: &mut Event<mctk_core::event::TouchDown>) {
        if self.disabled {
            return;
        }

        self.state_mut().pressed = true;
        self.state_mut().pressed_at = Some(Instant::now());
    }

    fn on_touch_up(&mut self, _event: &mut Event<mctk_core::event::TouchUp>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = false;
    }

    fn on_touch_drag_start(&mut self, _event: &mut Event<mctk_core::event::TouchDragStart>) {
        self.state_mut().pressed_at = None;
        self.state_mut().pressed = false;
    }

    fn view(&self) -> Option<Node> {
        let mut name: String = self.app.name.clone();
        let mut icon_path = self.app.icon_path.clone();
        let mut icon_box = node!(
            Div::new().border(Color::rgb(43., 43., 43.), 1.5, (0., 0., 0., 0.)),
            lay![
                 size: [88, 88],
                 axis_alignment: Alignment::Center,
                 cross_alignment: Alignment::Center,
                 margin: [0, 0, 14, 0],
                 padding: [7]
            ]
        );

        if let Some(path) = icon_path {
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("png") => {
                    icon_box = icon_box.push(node!(
                        Image::new(name.clone())
                            .dynamic_load_from(Some(path.to_str().unwrap().to_string())),
                        lay![
                            size_pct: [100],
                        ],
                    ));
                }
                Some("svg") => {
                    icon_box = icon_box.push(node!(
                        Svg::new(name.clone())
                            .dynamic_load_from(Some(path.to_str().unwrap().to_string())),
                        lay![
                            size_pct: [100],
                        ],
                    ));
                }
                _ => (),
            };
        } else {
            icon_box = icon_box.push(node!(
                Image::new(name.clone()).dynamic_load_from(Some(
                    "/usr/share/icons/Papirus-PNG/symbolic/apps/app-not-found.png".to_string()
                )),
                lay![
                    size_pct: [100],
                ],
            ));
        }

        Some(icon_box)
    }
}
