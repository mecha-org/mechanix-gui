use mctk_core::{
    component::{Component, Message},
    event::{Click, Event},
    lay, node, rect, size,
    style::Styled,
    widgets::{Div, IconButton, IconType},
    Color, Node,
};

pub struct PinnedApp {
    app_id: String,
    icon: String,
    on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    disabled: bool,
}

impl std::fmt::Debug for PinnedApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PinnedApp")
            .field("app_id", &self.app_id)
            .finish()
    }
}

impl PinnedApp {
    pub fn new<S: Into<String>>(app_id: S, icon: S) -> Self {
        Self {
            app_id: app_id.into(),
            icon: icon.into(),
            on_click: None,
            disabled: false,
        }
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}

impl Component for PinnedApp {
    fn on_click(&mut self, event: &mut Event<Click>) {
        if self.disabled {
            return;
        }

        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<Node> {
        let app_icon = node!(
            IconButton::new(self.app_id.clone())
                .icon_type(if self.icon.clone().ends_with(".svg") {
                    IconType::Svg
                } else {
                    IconType::Png
                })
                .with_class("btn-xxl border-0 p-4 rounded-xl")
                .disabled(self.disabled)
                .style("active_color", Color::rgba(42., 42., 44., 0.80)),
            lay![
                size: [Auto],
            ],
        );

        Some(
            node!(
                Div::new(),
                lay![
                     size: [88, 88],
                     margin: [0., 0., 0., 24]
                ]
            )
            .push(app_icon),
        )
    }
}
