use desktop_entries::DesktopEntry;
use mctk_core::{
    component::{Component, Message},
    event::{Click, Event},
    lay, node, rect, size,
    style::Styled,
    widgets::{Div, IconButton, IconType},
    Color, Node,
};

pub struct PinnedApp {
    app: DesktopEntry,
    on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    disabled: bool,
}

impl std::fmt::Debug for PinnedApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PinnedApp")
            .field("app_id", &self.app.app_id)
            .finish()
    }
}

impl PinnedApp {
    pub fn new(app: DesktopEntry) -> Self {
        Self {
            app,
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
        let mut start = node!(
            Div::new(),
            lay![
                 size: [88, 88],
                 margin: [0., 0., 0., 24]
            ]
        );

        if let Some(path) = self.app.icon_path.clone() {
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("png") => {
                    start = start.push(node!(
                        IconButton::new(self.app.name.clone())
                            .icon_type(IconType::Png)
                            .with_class("btn-xxl border-0 p-4 rounded-xl")
                            .disabled(self.disabled)
                            .style("active_color", Color::rgba(42., 42., 44., 0.80)),
                        lay![
                            size: [Auto],
                        ]
                    ));
                }
                Some("svg") => {
                    start = start.push(node!(
                        IconButton::new(self.app.name.clone())
                            .icon_type(IconType::Svg)
                            .with_class("btn-xxl border-0 p-4 rounded-xl")
                            .disabled(self.disabled)
                            .style("active_color", Color::rgba(42., 42., 44., 0.80)),
                        lay![
                            size: [Auto],
                        ]
                    ));
                }
                _ => (),
            };
        }

        Some(start)
    }
}
