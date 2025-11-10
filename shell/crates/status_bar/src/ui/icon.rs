use gpui::{prelude::FluentBuilder, *};
pub const STATUS_BAR_ICONS_DIR: &str = "icons/status-bar/";

#[derive(IntoElement, Clone)]
pub enum IconName {
    WirelessOn,
    WirelessOff,
    WireleessWarning,
    WirelessHigh,
    WirelessMedium,
    WirelessLow,
    BluetoothOn,
    BluetoothOff,
    BluetoothWarning,
    BluetoothConnected,
    Battery10,
    Battery20,
    Battery30,
    Battery40,
    Battery50,
    Battery60,
    Battery70,
    Battery80,
    Battery90,
    Battery100,
    BatteryEmpty,
    Battery10Charging,
    Battery20Charging,
    Battery30Charging,
    Battery40Charging,
    Battery50Charging,
    Battery60Charging,
    Battery70Charging,
    Battery80Charging,
    Battery90Charging,
    Battery100Charging,
}
impl IconName {
    pub fn resolve(&self) -> SharedString {
        let icon_path = match self {
            IconName::WirelessOn => "wireless-on.svg",
            IconName::WirelessOff => "wireless-off.svg",
            IconName::WireleessWarning => "wireless-warning.svg",
            IconName::WirelessHigh => "wireless-high.svg",
            IconName::WirelessMedium => "wireless-medium.svg",
            IconName::WirelessLow => "wireless-low.svg",
            IconName::BluetoothOn => "bluetooth-on.svg",
            IconName::BluetoothOff => "bluetooth-off.svg",
            IconName::BluetoothConnected => "bluetooth-connected.svg",
            IconName::BluetoothWarning => "bluetooth-warning.svg",
            IconName::Battery10 => "battery-10.svg",
            IconName::Battery20 => "battery-20.svg",
            IconName::Battery30 => "battery-30.svg",
            IconName::Battery40 => "battery-40.svg",
            IconName::Battery50 => "battery-50.svg",
            IconName::Battery60 => "battery-60.svg",
            IconName::Battery70 => "battery-70.svg",
            IconName::Battery80 => "battery-80.svg",
            IconName::Battery90 => "battery-90.svg",
            IconName::Battery100 => "battery-100.svg",
            IconName::BatteryEmpty => "battery-empty.svg",
            IconName::Battery10Charging => "battery-10-charging.svg",
            IconName::Battery20Charging => "battery-20-charging.svg",
            IconName::Battery30Charging => "battery-30-charging.svg",
            IconName::Battery40Charging => "battery-40-charging.svg",
            IconName::Battery50Charging => "battery-50-charging.svg",
            IconName::Battery60Charging => "battery-60-charging.svg",
            IconName::Battery70Charging => "battery-70-charging.svg",
            IconName::Battery80Charging => "battery-80-charging.svg",
            IconName::Battery90Charging => "battery-90-charging.svg",
            IconName::Battery100Charging => "battery-100-charging.svg",
        };
        format!("{}{}", STATUS_BAR_ICONS_DIR, icon_path).into()
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
