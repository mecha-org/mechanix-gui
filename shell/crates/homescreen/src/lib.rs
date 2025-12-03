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
        let state = HomescreenState::new(config);
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
        // Create widgets using grid bounds to showcase the implementation
        // Widget at grid position (0, 0), size 1x1
        self.state.create_widget(
            DemoWidget::new("Sunrise", rgb(0xff6b6b), rgb(0xff5252), true),
            0,
            Bounds {
                origin: point(0, 0),
                size: size(1, 1),
            },
        );

        // Widget at grid position (2, 0), size 2x2
        self.state.create_widget(
            DemoWidget::new("Ocean", rgb(0x4ecdc4), rgb(0x45b7aa), true),
            0,
            Bounds {
                origin: point(2, 0),
                size: size(2, 2),
            },
        );

        // Widget at grid position (0, 2), size 1x2
        self.state.create_widget(
            DemoWidget::new("Forest", rgb(0x95e1d3), rgb(0x7ed6c5), false),
            0,
            Bounds {
                origin: point(0, 2),
                size: size(1, 2),
            },
        );

        // Widget at grid position (3, 3), size 2x1
        self.state.create_widget(
            DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
            0,
            Bounds {
                origin: point(1, 3),
                size: size(2, 1),
            },
        );
        // Widget at grid position (3, 3), size 2x1
        self.state.create_widget(
            DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
            3,
            Bounds {
                origin: point(1, 3),
                size: size(2, 1),
            },
        );

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
