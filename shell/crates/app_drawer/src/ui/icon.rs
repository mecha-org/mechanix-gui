use gpui::{prelude::FluentBuilder, *};
use std::path::PathBuf;

pub const APP_DRAWER_ICONS_DIR: &str = "icons/app-drawer/";

#[derive(IntoElement, Clone, PartialEq, Debug)]
pub enum IconName {
    Category,
    Search,
    Close,
    DefaultApp,
}

impl IconName {
    pub fn resolve(self) -> SharedString {
        let icon_path = match self {
            Self::Category => "category.png",
            Self::Search => "search.png",
            Self::Close => "x.png",
            Self::DefaultApp => "default-app.png",
        };
        format!("{}{}", APP_DRAWER_ICONS_DIR, icon_path).into()
    }
}

impl RenderOnce for IconName {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::build(self)
    }
}

#[derive(IntoElement, Debug)]
pub struct Icon {
    path: SharedString,
    size: Option<(Pixels, Pixels)>,
    text_color: Option<Hsla>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            path: "".into(),
            size: None,
            text_color: None,
        }
    }
}

impl Icon {
    pub fn build(name: IconName) -> Self {
        Self::default().path(name.resolve())
    }

    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    pub fn text_color(mut self, text_color: impl Into<Hsla>) -> Self {
        self.text_color = Some(text_color.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl gpui::IntoElement {
        if self.path.starts_with("/") {
            let image_path: PathBuf = self.path.as_str().into();
            img(image_path).w(px(58.)).h(px(58.)).into_any_element()
        } else {
            img(self.path.clone())
                .w(px(58.))
                .h(px(58.))
                .into_any_element()
        }
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Self::build(name)
    }
}
