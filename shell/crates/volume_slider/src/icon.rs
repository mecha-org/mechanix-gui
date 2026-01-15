use gpui::{prelude::FluentBuilder, *};

pub const VOLUME_SLIDER_ICONS_DIR: &str = "icons/settings-drawer/";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeIconName {
    VolumeOff,
    VolumeLow,
    VolumeMedium,
    VolumeHigh,
}

impl VolumeIconName {
    /// Determine the appropriate icon based on volume level (0-100)
    pub fn from_volume(volume: f32, min: f32, max: f32) -> Self {
        if volume <= min {
            VolumeIconName::VolumeOff
        } else {
            let range = max - min;
            let normalized = (volume - min) / range;
            if normalized <= 0.33 {
                VolumeIconName::VolumeLow
            } else if normalized <= 0.66 {
                VolumeIconName::VolumeMedium
            } else {
                VolumeIconName::VolumeHigh
            }
        }
    }

    pub fn resolve(self) -> SharedString {
        let icon_path = match self {
            VolumeIconName::VolumeOff => "volume-off.svg",
            VolumeIconName::VolumeLow => "volume-low.svg",
            VolumeIconName::VolumeMedium => "volume-medium.svg",
            VolumeIconName::VolumeHigh => "volume-high.svg",
        };

        format!("{}{icon_path}", VOLUME_SLIDER_ICONS_DIR).into()
    }
}

#[derive(IntoElement)]
pub struct VolumeIcon {
    main: Svg,
    path: SharedString,
    size: Option<(Pixels, Pixels)>,
    text_color: Option<Hsla>,
}

impl Default for VolumeIcon {
    fn default() -> Self {
        Self {
            main: svg(),
            path: "".into(),
            size: None,
            text_color: None,
        }
    }
}

impl VolumeIcon {
    pub fn new(name: VolumeIconName) -> Self {
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

impl RenderOnce for VolumeIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.main
            .path(self.path)
            .w(px(25.))
            .h(px(25.))
            .when_some(self.text_color, |this, text_color| {
                this.text_color(text_color)
            })
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
    }
}

#[allow(dead_code)]
pub const LOCKSCREEN_ICONS_DIR: &str = "icons/lockscreen/";

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LockscreenIconName {
    Arrow,
    Bell,
    Lock,
    LockOpenHalf,
    LockOpenFull,
    WedgeLeft,
    WedgeLeftOutline,
    WedgeRight,
    WedgeRightOutline,
}

impl LockscreenIconName {
    #[allow(dead_code)]
    pub fn resolve(self) -> SharedString {
        let icon_path = match self {
            LockscreenIconName::Arrow => "arrow.svg",
            LockscreenIconName::Bell => "bell.svg",
            LockscreenIconName::Lock => "lock.svg",
            LockscreenIconName::LockOpenHalf => "lock-open-half.svg",
            LockscreenIconName::LockOpenFull => "lock-open-full.svg",
            LockscreenIconName::WedgeLeft => "wedge_left.svg",
            LockscreenIconName::WedgeLeftOutline => "wedge_left_outline.svg",
            LockscreenIconName::WedgeRight => "wedge_right.svg",
            LockscreenIconName::WedgeRightOutline => "wedge_right_outline.svg",
        };

        format!("{}{icon_path}", LOCKSCREEN_ICONS_DIR).into()
    }
}

#[allow(dead_code)]
#[derive(IntoElement)]
pub struct LockscreenIcon {
    main: Svg,
    path: SharedString,
    size: Option<(Pixels, Pixels)>,
    text_color: Option<Hsla>,
}

#[allow(dead_code)]
impl Default for LockscreenIcon {
    fn default() -> Self {
        Self {
            main: svg(),
            path: "".into(),
            size: None,
            text_color: None,
        }
    }
}

#[allow(dead_code)]
impl LockscreenIcon {
    pub fn new(name: LockscreenIconName) -> Self {
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

#[allow(dead_code)]
impl RenderOnce for LockscreenIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.main
            .path(self.path)
            .w(px(25.))
            .h(px(25.))
            .when_some(self.text_color, |this, text_color| {
                this.text_color(text_color)
            })
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
    }
}
