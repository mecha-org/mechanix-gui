use gpui::{prelude::FluentBuilder, *};
pub const UNIVERSAL_SEARCH_ICONS_DIR: &str = "icons/universal-search/";

#[derive(IntoElement, Clone)]
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
}
impl IconName {
    pub fn resolve(&self) -> SharedString {
        let icon_path = match self {
            IconName::Ardour => "ardour_icon.svg",
            IconName::ArrowUpRight => "arrow_up_right_icon.svg",
            IconName::Firefox => "firefox_icon.svg",
            IconName::Chromium => "chromium_icon.svg",
            IconName::Github => "github_icon.svg",
            IconName::FolderMedium => "folder_medium_icon.svg",
            IconName::Search => "search_icon.svg",
            IconName::FolderSmall => "folder_small_icon.svg",
            IconName::XIcon => "x_icon.svg",
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
    main: Svg,
    path: SharedString,
    size: Option<(Pixels, Pixels)>,
    text_color: Option<Hsla>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            main: svg(),
            path: "".into(),
            size: None,
            text_color: None,
        }
    }
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self::default().path(name.resolve())
    }

    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    fn build(name: IconName) -> Self {
        Self::default().path(name.resolve())
    }

    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
        self
    }

    pub fn text_color(mut self, text_color: impl Into<Hsla>) -> Self {
        self.text_color = Some(text_color.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.main
            .path(self.path)
            .w(px(40.))
            .h(px(40.))
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
            .when_some(self.text_color, |this, text_color| {
                this.text_color(text_color)
            })
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Self::build(name)
    }
}
