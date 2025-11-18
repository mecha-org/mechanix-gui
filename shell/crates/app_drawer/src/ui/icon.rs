use gpui::{prelude::FluentBuilder, *};

pub const APP_DRAWER_ICONS_DIR: &str = "icons/app-drawer/";

#[derive(IntoElement, Clone, PartialEq, Debug)]
pub enum IconName {
    Telegram,
    Mecha,
    Chromium,
    Files,
    Firefox,
    Category,
    Search,
    Close,
}

impl IconName {
    pub fn resolve(self) -> SharedString {
        let icon_path = match self {
            Self::Telegram => "telegram.png",
            Self::Mecha => "mecha.png",
            Self::Chromium => "chromium.png",
            Self::Files => "files.png",
            Self::Firefox => "firefox.png",
            Self::Category => "category.png",
            Self::Search => "search.png",
            Self::Close => "x.png",
        };
        format!("{}{}", APP_DRAWER_ICONS_DIR, icon_path).into()
    }
}

impl RenderOnce for IconName {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::build(self)
    }
}

#[derive(IntoElement)]
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
        let is_svg = self.path.ends_with(".svg");

        if is_svg {
            // SVG icon case
            let base = svg().path(self.path.clone()).w(px(40.)).h(px(40.));

            // Apply color if available
            let rendered = base.when_some(self.text_color, |this, color| this.text_color(color));

            rendered.into_any_element()
        } else {
            // PNG / JPEG icon case
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
