use gpui::{prelude::FluentBuilder, *};

#[derive(IntoElement, Clone)]
pub enum IconName {
    RotationOn,
    RotationOff,
    Airplane,
    ScreenMirroringOn,
    ScreenMirroringOff,
    Battery40,
    MicroPhoneOff,
    MicroPhoneOn,
    ScreenRecordingOff,
    ScreenRecordingOn,
    Calculator,
    Camera,
}

impl IconName {
    pub fn resolve(self) -> SharedString {
        match self {
            Self::RotationOn => "icons/rotation-on.svg",
            Self::RotationOff => "icons/rotation-off.svg",
            Self::Airplane => "icons/airplane.svg",
            Self::ScreenMirroringOn => "icons/screen-mirroring-on.svg",
            Self::ScreenMirroringOff => "icons/screen-mirroring-off.svg",
            Self::Battery40 => "icons/battery-40.svg",
            Self::MicroPhoneOff => "icons/microphone-off.svg",
            Self::MicroPhoneOn => "icons/microphone-on.svg",
            Self::ScreenRecordingOff => "icons/screen-recording-off.svg",
            Self::ScreenRecordingOn => "icons/screen-recording-on.svg",
            Self::Calculator => "icons/calculator.svg",
            Self::Camera => "icons/camera.svg",
        }
        .into()
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
