use gpui::{prelude::FluentBuilder, *};
pub const UNIVERSAL_SEARCH_ICONS_DIR: &str = "icons/universal-search/";

#[derive(IntoElement, Clone, PartialEq, Debug)]
pub enum IconName {
    Ardour,
    ArrowUpRight,
    Chromium,
    Firefox,
    Github,
    FolderMedium,
    Search,
    FolderSmall,
    XIcon,
    File,
    ArrowCounterClockWise,
    Json,
    Rust,
    TypeScript,
    Markdown,
}

impl IconName {
    pub fn resolve(&self) -> SharedString {
        let icon_path = match self {
            IconName::Ardour => "ardour-icon.png",
            IconName::ArrowUpRight => "arrow-up-right-icon.svg",
            IconName::Firefox => "firefox-icon.png",
            IconName::Chromium => "chromium-icon.png",
            IconName::Github => "github-icon.png",
            IconName::FolderMedium => "folder-medium-icon.svg",
            IconName::Search => "search-icon.svg",
            IconName::FolderSmall => "folder-small-icon.svg",
            IconName::XIcon => "x-icon.svg",
            IconName::File => "file-icon.png",
            IconName::ArrowCounterClockWise => "arrow-counter-clock-wise.svg",
            IconName::Json => "json-icon.png",
            IconName::Rust => "rust-icon.png",
            IconName::TypeScript => "typescript-icon.png",
            IconName::Markdown => "markdown-icon.png",
        };
        format!("{}{}", UNIVERSAL_SEARCH_ICONS_DIR, icon_path).into()
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
    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl gpui::IntoElement {
        let is_svg = self.path.ends_with(".svg");
        if is_svg {
            // SVG icon case
            let base = svg().path(self.path.clone()).w(px(40.0)).h(px(40.0));

            // Apply color if available
            let rendered = base
                .when_some(self.text_color, |this, color| this.text_color(color))
                .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1));

            rendered.into_any_element()
        } else {
            // PNG / JPEG icon case
            img(self.path.clone())
                .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
                .into_any_element()
        }
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Self::build(name)
    }
}
