use crate::widgets::HomescreenWidget;
use dispatcher::{self, Dispatcher};
use gpui::*;

/// Global state for extension widget that can be updated from listen_for_extensions
#[derive(Clone)]
pub struct ExtensionState {
    pub kind: Option<ExtensionKind>,
}

impl Default for ExtensionState {
    fn default() -> Self {
        Self { kind: None }
    }
}

impl Global for ExtensionState {}

pub struct ExtensionWidget {
    bounds: Bounds<Pixels>,
    has_border: bool,
}

#[derive(Clone)]
pub enum ExtensionKind {
    Gamepad,
    Keyboard,
    Gpio,
    Unknown
}

impl ExtensionKind {
    pub fn get_icon_path(&self) -> &str {
        match self {
            ExtensionKind::Gamepad => "icons/homescreen/extension/gamepad_icon.svg",
            ExtensionKind::Keyboard => "icons/homescreen/extension/keyboard_icon.svg",
            ExtensionKind::Gpio => "icons/homescreen/extension/gpio_icon.svg",
            ExtensionKind::Unknown => "icons/homescreen/extension/unknown_icon.svg",
        }
    }

    pub fn get_background_path(&self) -> &str {
        match self {
            ExtensionKind::Gamepad => "icons/homescreen/extension/gamepad_background.svg",
            ExtensionKind::Keyboard => "icons/homescreen/extension/keyboard_background.svg",
            ExtensionKind::Gpio => "icons/homescreen/extension/gpio_background.svg",
            ExtensionKind::Unknown => "icons/homescreen/extension/unknown_background.svg",
        }
    }

    pub fn from(id: &str) -> Self {
        match id {
            "GAMEPAD" => ExtensionKind::Gamepad,
            "KEYBOARD" => ExtensionKind::Keyboard,
            "GPIO" => ExtensionKind::Gpio,
            _ => ExtensionKind::Unknown,
        }
    }
}

impl ExtensionWidget {
    pub fn new(has_border: bool) -> Self {
        Self {
            bounds: Bounds::default(),
            has_border,
        }
    }
}

impl Default for ExtensionWidget {
    fn default() -> Self {
        Self::new(true)
    }
}

impl HomescreenWidget for ExtensionWidget {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        // Get the current extension kind from global state
        let extension_state = if cx.has_global::<ExtensionState>() {
            cx.global::<ExtensionState>().clone()
        } else {
            ExtensionState::default()
        };

        // Asset paths for layered rendering based on extension kind
        let (icon_path, background_path) = match &extension_state.kind {
            Some(kind) => (kind.get_icon_path(), kind.get_background_path()),
            None => (
                "icons/homescreen/extension/detached_icon.svg",
                "icons/homescreen/extension/detached_background.svg",
            ),
        };
        let dot_grid_path = "icons/homescreen/extension/dot_grid.svg";

        div()
            .size_full()
            .relative()
            .overflow_hidden()
            // Layer 1: Background SVG (bottom layer)
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(img(background_path).size_full())
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(img(dot_grid_path).size_full()),
                    ),
            )
            // Layer 2: Dot grid SVG (middle layer)
            // Layer 3: Icon/gamepad image (top layer, centered)
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(img(icon_path)),
            )
            .into_any_element()
    }

    fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    fn get_bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn background_color(&self) -> Hsla {
        rgb(0x2a2a2a).into()
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        rgb(0x404040).into()
    }
}

pub fn listen_for_extensions(cx: &mut App) {
    // Initialize the global ExtensionState if not already set
    if !cx.has_global::<ExtensionState>() {
        cx.set_global(ExtensionState::default());
    }

    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    cx.spawn(async move |app| {
        while let Ok(message) = dispatcher_rx.recv().await {
            match message {
                dispatcher::Message::ExtensionAttached(id) => {
                    let _ = app.update(|cx| {
                        println!("Extension attached: {id}");
                        let kind = ExtensionKind::from(&id);
                        cx.global_mut::<ExtensionState>().kind = Some(kind);
                        cx.refresh_windows();
                    });
                }
                dispatcher::Message::ExtensionDetached(id) => {
                    let _ = app.update(|cx| {
                        println!("Extension detached: {id}");
                        cx.global_mut::<ExtensionState>().kind = None;
                        cx.refresh_windows();
                    });
                }
                _ => {}
            }
        }
    })
    .detach();
}
