use gpui::{prelude::FluentBuilder, *};
pub const SETTINGS_DRAWER_ICONS_DIR: &str = "icons/settings-drawer/";

#[derive(IntoElement, Clone)]
pub enum IconName {
    Settings,
    Power,
    HighPower,
    SavingPower,
    RotationOn,
    RotationOff,
    Airplane,
    ScreenMirroringOn,
    ScreenMirroringOff,
    PowerModeHigh,
    PowerModeBalanced,
    PowerModeLow,
    MicroPhoneOn,
    MicroPhoneOff,
    ScreenRecordingOn,
    ScreenRecordingOff,
    CameraOff,
    CameraOn,
    BrightnessLow,
    BrightnessMedium,
    BrightnessHigh,
    VolumeLow,
    VolumeMedium,
    VolumeHigh,
    VolumeOff,
    WirelessOff,
    WirelessWarning,
    WirelessOn,
    WirelessHigh,
    WirelessMedium,
    WirelessLow,
    WirelessHighLocked,
    WirelessMediumLocked,
    WirelessLowLocked,
    ConnectedWirelessWarning,
    ConnectedWirelessOn,
    ConnectedWirelessHigh,
    ConnectedWirelessMedium,
    ConnectedWirelessLow,
    ConnectedWirelessHighLocked,
    ConnectedWirelessMediumLocked,
    ConnectedWirelessLowLocked,
    BluetoothOn,
    BluetoothOff,
    BluetoothConnected,
    Terminal,
    CellSignalNone,
    CellSignalHigh,
    CellSignalWarning,
    Navbar,
    ConnectedIcon,
    SystemSpeaker,
    Headphone,
    ExternalSpeaker,
    ExtendedDetected,
    MirrorScreen,
    ExtendedOnly,
    SecondScreen,
    AutoBrightness,
    DarkMode,
}

impl IconName {
    pub fn resolve(self) -> SharedString {
        let icon_path = match self {
            IconName::Settings => "settings.svg",
            IconName::Power => "power.svg",
            IconName::HighPower => "high-performance.svg",
            IconName::SavingPower => "low-performance.svg",
            IconName::PowerModeHigh => "power-mode-high.svg",
            IconName::PowerModeBalanced => "power-mode-balanced.svg",
            IconName::PowerModeLow => "power-mode-low.svg",
            IconName::RotationOn => "rotation-on.svg",
            IconName::RotationOff => "rotation-off.svg",
            IconName::Airplane => "airplane.svg",
            IconName::ScreenMirroringOn => "screen-mirroring-on.svg",
            IconName::ScreenMirroringOff => "screen-mirroring-off.svg",
            IconName::MicroPhoneOn => "microphone-on.svg",
            IconName::MicroPhoneOff => "microphone-off.svg",
            IconName::ScreenRecordingOn => "screen-recording-on.svg",
            IconName::ScreenRecordingOff => "screen-recording-off.svg",
            IconName::CameraOff => "camera-off.svg",
            IconName::CameraOn => "camera-on.svg",
            IconName::BrightnessLow => "brightness-low.svg",
            IconName::BrightnessMedium => "brightness-medium.svg",
            IconName::BrightnessHigh => "brightness-high.svg",
            IconName::VolumeLow => "volume-low.svg",
            IconName::VolumeMedium => "volume-medium.svg",
            IconName::VolumeHigh => "volume-high.svg",
            IconName::VolumeOff => "volume-off.svg",
            IconName::WirelessOff => "wireless-off.svg",
            IconName::WirelessWarning => "wireless-warning.svg",
            IconName::WirelessOn => "wireless-none.svg",
            IconName::WirelessHigh => "wireless-high.svg",
            IconName::WirelessMedium => "wireless-medium.svg",
            IconName::WirelessLow => "wireless-low.svg",
            IconName::WirelessHighLocked => "wireless-high-locked.svg",
            IconName::WirelessMediumLocked => "wireless-medium-locked.svg",
            IconName::WirelessLowLocked => "wireless-low-locked.svg",
            IconName::ConnectedWirelessWarning => "connected-wireless-warning.svg",
            IconName::ConnectedWirelessOn => "connected-wireless-on.svg",
            IconName::ConnectedWirelessHigh => "connected-wireless-high.svg",
            IconName::ConnectedWirelessMedium => "connected-wireless-medium.svg",
            IconName::ConnectedWirelessLow => "connected-wireless-low.svg",
            IconName::ConnectedWirelessHighLocked => "connected-wireless-high-locked.svg",
            IconName::ConnectedWirelessMediumLocked => "connected-wireless-medium-locked.svg",
            IconName::ConnectedWirelessLowLocked => "connected-wireless-low-locked.svg",
            IconName::BluetoothOn => "bluetooth-on.svg",
            IconName::BluetoothOff => "bluetooth-off.svg",
            IconName::BluetoothConnected => "bluetooth-connected.svg",
            IconName::Terminal => "terminal.svg",
            IconName::CellSignalNone => "cell-signal-none.svg",
            IconName::CellSignalHigh => "cell-signal-high.svg",
            IconName::CellSignalWarning => "cell-signal-warning.svg",
            IconName::Navbar => "navbar.png",
            IconName::ConnectedIcon => "connected_icon.svg",
            IconName::SystemSpeaker => "system-speaker.svg",
            IconName::Headphone => "headphone.svg",
            IconName::ExternalSpeaker => "external-speaker.svg",
            IconName::ExtendedDetected => "extended-detected.svg",
            IconName::MirrorScreen => "mirror-screen.svg",
            IconName::ExtendedOnly => "extended-only.svg",
            IconName::SecondScreen => "second-screen.svg",
            IconName::AutoBrightness => "auto-brightness.svg",
            IconName::DarkMode => "dark-mode.svg",
        };

        format!("{}{icon_path}", SETTINGS_DRAWER_ICONS_DIR).into()
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

    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
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
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Self::build(name)
    }
}
