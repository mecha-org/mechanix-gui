use crate::widgets::HomescreenWidget;
use dispatcher::{self, Dispatcher};
use gpui::*;
use icons::prelude::Icons;
use theme::ActiveTheme;

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
    background_color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

#[derive(Clone)]
pub enum ExtensionKind {
    Gamepad,
    Keyboard,
    Gpio,
    Unknown,
}

impl ExtensionKind {
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
    pub fn new(color: impl Into<Hsla>, border_color: impl Into<Hsla>, has_border: bool) -> Self {
        Self {
            bounds: Bounds::default(),
            background_color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
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

        let icons = Icons::global(cx).homescreen.extensions.clone();
        let colors = cx.theme().colors.clone();

        // Asset paths for layered rendering based on extension kind
        let extension_icon = match &extension_state.kind {
            Some(kind) => match kind {
                ExtensionKind::Gamepad => icons.gamepad,
                ExtensionKind::Keyboard => icons.keyboard,
                ExtensionKind::Gpio => icons.gpio,
                ExtensionKind::Unknown => icons.unknown,
            },
            None => icons.detached,
        };

        let background = icons.dot_grid.clone();

        div()
            .size_full()
            .relative()
            .overflow_hidden()
            .rounded(px(16.))
            // Layer 1: Background SVG (bottom layer)
            .child(
                div().absolute().inset_0().child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .justify_center()
                        .items_center()
                        .child(img(background).size_full()),
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
                    .child(img(extension_icon)),
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
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
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
                dispatcher::Message::SetExtensionDetected(id) => {
                    let _ = app.update(|cx| {
                        println!("Extension attached: {id}");
                        let kind = ExtensionKind::from(&id);
                        cx.global_mut::<ExtensionState>().kind = Some(kind);
                        cx.refresh_windows();
                    });
                }
                dispatcher::Message::SetExtensionName(id) => {
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
