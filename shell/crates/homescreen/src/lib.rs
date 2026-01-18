use commons::input::{
    Backspace, Copy, Cut, Delete, End, Home, Left, Paste, Right, SelectAll, SelectLeft,
    SelectRight, ShowCharacterPalette,
};
use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use settings::prelude::*;
use status_bar::prelude::status_bar_components;
use theme::ActiveTheme;

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
use crate::widgets::app_drawer::AppDrawerWidget;
use crate::widgets::demo_widget::DemoWidget;
use crate::widgets::universal_search::UniversalSearchWidget;

pub struct Homescreen {
    state: HomescreenState,
    status_bar_size: Size<Pixels>,
}

impl Homescreen {
    pub fn new(
        _cx: &mut Context<Self>,
        config: HomescreenConfig,
        status_bar_size: Size<Pixels>,
    ) -> Self {
        let mut state = HomescreenState::new(config);
        state.create_widget(
            UniversalSearchWidget::new(_cx),
            0,
            Bounds {
                origin: point(0, 0),
                size: size(4, 4), // Adjust size as needed for a search bar
            },
        );
        // state.create_widget(
        //     DemoWidget::new("Sunrise", rgb(0xff6b6b), rgb(0xff5252), true),
        //     0,
        //     Bounds {
        //         origin: point(0, 0),
        //         size: size(1, 1),
        //     },
        // );
        //
        // state.create_widget(
        //     DemoWidget::new("Ocean", rgb(0x4ecdc4), rgb(0x45b7aa), true),
        //     0,
        //     Bounds {
        //         origin: point(2, 0),
        //         size: size(2, 2),
        //     },
        // );
        //
        // state.create_widget(
        //     DemoWidget::new("Forest", rgb(0x95e1d3), rgb(0x7ed6c5), false),
        //     0,
        //     Bounds {
        //         origin: point(0, 2),
        //         size: size(1, 2),
        //     },
        // );
        //
        // state.create_widget(
        //     DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
        //     0,
        //     Bounds {
        //         origin: point(1, 3),
        //         size: size(2, 1),
        //     },
        // );
        //
        // state.create_widget(
        //     DemoWidget::new("Sky", rgb(0x5fa8d3), rgb(0x4a90bb), true),
        //     0,
        //     Bounds {
        //         origin: point(1, 0),
        //         size: size(1, 2),
        //     },
        // );

        // PAGE 1 - First row: 2x1 + 1x1 + 1x1, then 4x3 widget
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
                origin: point(3, 0),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Plum", rgb(0xb565a7), rgb(0x9d5091), true),
            1,
            Bounds {
                origin: point(0, 1),
                size: size(4, 3),
            },
        );

        // PAGE 2 - Two 4x2 widgets
        state.create_widget(
            DemoWidget::new("Rose", rgb(0xff6b9d), rgb(0xff5285), true),
            2,
            Bounds {
                origin: point(0, 0),
                size: size(4, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Teal", rgb(0x1abc9c), rgb(0x16a085), true),
            2,
            Bounds {
                origin: point(0, 2),
                size: size(4, 2),
            },
        );

        // PAGE 3 - Top row: 3x1 + 1x1, middle: 4x2, bottom row: 2x2 + 2x2
        state.create_widget(
            DemoWidget::new("Tangerine", rgb(0xff9500), rgb(0xe68200), true),
            3,
            Bounds {
                origin: point(0, 0),
                size: size(3, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Aqua", rgb(0x00bcd4), rgb(0x00a3ba), true),
            3,
            Bounds {
                origin: point(3, 0),
                size: size(1, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Mauve", rgb(0xe0b0ff), rgb(0xc78fff), false),
            3,
            Bounds {
                origin: point(0, 1),
                size: size(4, 2),
            },
        );

        state.create_widget(
            DemoWidget::new("Gold", rgb(0xffd700), rgb(0xe6c200), true),
            3,
            Bounds {
                origin: point(0, 3),
                size: size(2, 1),
            },
        );

        state.create_widget(
            DemoWidget::new("Lavender", rgb(0xc7b3ff), rgb(0xb59fff), true),
            3,
            Bounds {
                origin: point(2, 3),
                size: size(2, 1),
            },
        );

        state.create_widget(
            AppDrawerWidget::new(_cx),
            4,
            Bounds {
                origin: point(0, 0),
                size: size(4, 4), // Full screen widget
            },
        );

        Self {
            state,
            status_bar_size,
        }
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
        let status_bar_size = self.status_bar_size;
        let is_animating = AnimationManager::animate(&mut self.state);
        if is_animating {
            window.request_animation_frame();
        }
        let colors = cx.theme().colors.clone();

        div()
            .size_full()
            .bg(colors.background_1000)
            .flex()
            .flex_col()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .child(status_bar_components(cx, status_bar_size, true))
            .child(HomescreenUi::render(&self.state, cx))
    }
}

pub mod prelude {
    pub use crate::config::*;
    pub use crate::run_app;
    pub use crate::Homescreen;
}

pub fn run_app(cx: &mut App) {
    let HomescreenSettings {
        navbar_height,
        status_bar_size,
        layer_shell,
        ..
    } = Settings::global(cx).homescreen.clone();

    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        ..
    } = layer_shell;
    let screen_size = gpui::size(size.width, size.height - navbar_height);

    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, screen_size, cx));
    let config = HomescreenConfig::new(screen_size);

    // Register key bindings for the text input
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, None),
        KeyBinding::new("delete", Delete, None),
        KeyBinding::new("left", Left, None),
        KeyBinding::new("right", Right, None),
        KeyBinding::new("shift-left", SelectLeft, None),
        KeyBinding::new("shift-right", SelectRight, None),
        KeyBinding::new("cmd-a", SelectAll, None),
        KeyBinding::new("ctrl-a", SelectAll, None), // Add Windows/Linux alternative
        KeyBinding::new("home", Home, None),
        KeyBinding::new("end", End, None),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        KeyBinding::new("cmd-v", Paste, None),
        KeyBinding::new("ctrl-v", Paste, None), // Add Windows/Linux alternative
        KeyBinding::new("cmd-c", Copy, None),
        KeyBinding::new("ctrl-c", Copy, None), // Add Windows/Linux alternative
        KeyBinding::new("cmd-x", Cut, None),
        KeyBinding::new("ctrl-x", Cut, None), // Add Windows/Linux alternative
    ]);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::None,
                margin: None,
                exclusive_zone: None,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let mut regions = Vec::new();
            regions.push(Bounds {
                origin: point(px(0.), px(0.)),
                size: size,
            });
            window.set_input_regions(Some(regions));

            cx.new(|cx| Homescreen::new(cx, config, status_bar_size))
        },
    )
    .unwrap();
}
