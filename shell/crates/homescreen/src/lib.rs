use gpui::*;

mod animation_manager;
mod config;
mod input_manager;
mod layout_manager;
mod state;
mod ui;
mod utils;
mod widgets;

use crate::animation_manager::AnimationManager;
use crate::config::HomescreenConfig;
use crate::input_manager::InputManager;
use crate::state::*;
use crate::ui::HomescreenUi;
use crate::widgets::demo_widget::DemoWidget;

pub struct Homescreen {
    state: HomescreenState,
}

impl Homescreen {
    pub fn new(_cx: &mut Context<Self>, config: HomescreenConfig) -> Self {
        let mut state = HomescreenState::new(config);
        state.create_widget(
            DemoWidget::new("Sunrise", rgb(0xff6b6b), rgb(0xff5252), true),
            0,
            Bounds {
                origin: point(0, 0),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Ocean", rgb(0x4ecdc4), rgb(0x45b7aa), true),
            0,
            Bounds {
                origin: point(2, 0),
                size: size(2, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Forest", rgb(0x95e1d3), rgb(0x7ed6c5), false),
            0,
            Bounds {
                origin: point(0, 2),
                size: size(1, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
            0,
            Bounds {
                origin: point(1, 3),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Sky", rgb(0x5fa8d3), rgb(0x4a90bb), true),
            0,
            Bounds {
                origin: point(1, 0),
                size: size(1, 2),
            },
        );

        // PAGE 1 - New widgets
        state.create_widget(
            DemoWidget::new("Coral", rgb(0xff7f50), rgb(0xff6347), true),
            1,
            Bounds {
                origin: point(0, 0),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Mint", rgb(0x98d8c8), rgb(0x7ac7b5), false),
            1,
            Bounds {
                origin: point(2, 0),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Amber", rgb(0xffa94d), rgb(0xff8c1a), true),
            1,
            Bounds {
                origin: point(0, 1),
                size: size(1, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Plum", rgb(0xb565a7), rgb(0x9d5091), true),
            1,
            Bounds {
                origin: point(1, 1),
                size: size(2, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Sage", rgb(0xa8c69f), rgb(0x8fb386), false),
            1,
            Bounds {
                origin: point(1, 3),
                size: size(1, 1),
            },
        );

        // PAGE 2 - New widgets
        state.create_widget(
            DemoWidget::new("Rose", rgb(0xff6b9d), rgb(0xff5285), true),
            2,
            Bounds {
                origin: point(0, 0),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Teal", rgb(0x1abc9c), rgb(0x16a085), true),
            2,
            Bounds {
                origin: point(1, 0),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Peach", rgb(0xffdab9), rgb(0xffc99f), false),
            2,
            Bounds {
                origin: point(0, 1),
                size: size(1, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Indigo", rgb(0x6a5acd), rgb(0x5a4ab3), true),
            2,
            Bounds {
                origin: point(1, 1),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Lime", rgb(0xcddc39), rgb(0xb3c427), true),
            2,
            Bounds {
                origin: point(2, 1),
                size: size(2, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Blush", rgb(0xffb3ba), rgb(0xff99a1), false),
            2,
            Bounds {
                origin: point(1, 3),
                size: size(1, 1),
            },
        );

        // PAGE 3 - Keep existing and add more
        state.create_widget(
            DemoWidget::new("Tangerine", rgb(0xff9500), rgb(0xe68200), true),
            3,
            Bounds {
                origin: point(0, 0),
                size: size(1, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Aqua", rgb(0x00bcd4), rgb(0x00a3ba), true),
            3,
            Bounds {
                origin: point(1, 0),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Mauve", rgb(0xe0b0ff), rgb(0xc78fff), false),
            3,
            Bounds {
                origin: point(3, 0),
                size: size(1, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Gold", rgb(0xffd700), rgb(0xe6c200), true),
            3,
            Bounds {
                origin: point(1, 1),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
            3,
            Bounds {
                origin: point(1, 3),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Olive", rgb(0x9aad7e), rgb(0x84976a), true),
            3,
            Bounds {
                origin: point(0, 2),
                size: size(1, 1),
            },
        );

        Self { state }
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        InputManager::mouse_down(event, &mut self.state);
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if InputManager::mouse_move(event, &mut self.state) {
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        InputManager::mouse_up(event, &mut self.state);
        cx.notify();
    }
}

impl Render for Homescreen {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let is_animating = AnimationManager::animate(&mut self.state);
        if is_animating {
            window.request_animation_frame();
        }

        div()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .child(HomescreenUi::render(&self.state))
    }
}

pub mod prelude {
    pub use crate::config::*;
    pub use crate::Homescreen;
}
