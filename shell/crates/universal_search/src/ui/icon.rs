use std::path::PathBuf;

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
    Navbar,
    AudioFile,
    CodeFile,
    CsvFile,
    DefaultFile,
    DefaultApp,
    DocFile,
    ImageFile,
    LockedFile,
    PdfFile,
    VideoFile,
    XlsFile,
    ZipFile,
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
            IconName::Navbar => "navbar.png",
            IconName::AudioFile => "audio-file-icon.svg",
            IconName::CodeFile => "code-file-icon.svg",
            IconName::CsvFile => "csv-file-icon.svg",
            IconName::DefaultFile => "default-file-icon.svg",
            IconName::DefaultApp => "default-app-icon.svg",
            IconName::DocFile => "doc-file-icon.svg",
            IconName::ImageFile => "image-file-icon.svg",
            IconName::LockedFile => "locked-file-icon.svg",
            IconName::PdfFile => "pdf-file-icon.svg",
            IconName::VideoFile => "video-file-icon.svg",
            IconName::XlsFile => "xls-file-icon.svg",
            IconName::ZipFile => "zip-file-icon.svg",
        };
        format!("{}{}", UNIVERSAL_SEARCH_ICONS_DIR, icon_path).into()
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
    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl gpui::IntoElement {
        let is_svg = self.path.ends_with(".svg");
        let is_absolute = self.path.starts_with("/");

        // SVG HANDLING (tint only for relative assets)
        if is_svg {
            if !is_absolute {
                // Relative SVG → render with optional color
                let svg_el = svg().path(self.path.clone()).w(px(40.)).h(px(40.));

                let tinted = svg_el
                    .when_some(self.text_color, |this, color| this.text_color(color))
                    .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1));

                return tinted.into_any_element();
            }
        }

        if is_absolute {
            return img(PathBuf::from(self.path.as_str()))
                .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
                .into_any_element();
        }

        img(self.path.clone())
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
            .into_any_element()
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Self::build(name)
    }
}
