use gpui::{ prelude::FluentBuilder, * };

pub const RUNNING_APPS_DIR: &str = "icons/running-apps/";

#[derive(IntoElement, Clone, PartialEq, Debug)]
pub enum IconName {
    CleanUp,
    BgApp,
}

impl IconName {
    pub fn resolve(&self) -> SharedString {
        let icon_path = match self {
            IconName::CleanUp => "cleanup.svg",
            IconName::BgApp => "bg-app.png",
        };
        format!("{}{}", RUNNING_APPS_DIR, icon_path).into()
    }
}

impl RenderOnce for IconName {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        Icon::from(self).render(window, cx)
    }
}

#[derive(IntoElement, Clone)]
pub struct Icon {
    path: Option<SharedString>,
    size: Option<(Pixels, Pixels)>,
    text_color: Option<Hsla>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            path: None,
            size: None,
            text_color: None,
        }
    }
}

impl Icon {
    // Build from IconName
    pub fn from_icon_name(name: IconName) -> Self {
        let resolved = name.resolve();
        if resolved.is_empty() {
            Self::default()
        } else {
            Self::default().path(resolved)
        }
    }

    // Build from string path
    pub fn from_path(path: impl Into<SharedString>) -> Self {
        let p: SharedString = path.into();
        if p.is_empty() {
            Self::default()
        } else {
            Self::default().path(p)
        }
    }

    // Builder methods
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        let p = path.into();
        self.path = if p.is_empty() { None } else { Some(p) };
        self
    }

    pub fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // No path = empty div
        let Some(path) = self.path.clone() else {
            return div().into_any_element();
        };

        let is_svg = path.ends_with(".svg");

        if is_svg {
            let base = svg().path(path.clone());

            let rendered = base
                .when_some(self.text_color, |this, c| this.text_color(c))
                .when_some(self.size, |this, s| this.w(s.0).h(s.1));

            rendered.into_any_element()
        } else {
            img(path.clone())
                .when_some(self.size, |this, s| this.w(s.0).h(s.1))
                .into_any_element()
        }
    }
}

// Support IconName → Icon conversion
impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Icon::from_icon_name(name)
    }
}

// Support SharedString → Icon
impl From<SharedString> for Icon {
    fn from(path: SharedString) -> Self {
        Icon::from_path(path)
    }
}
