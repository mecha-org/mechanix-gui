use gpui::{prelude::FluentBuilder, *};

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
    pub fn resolve(self) -> SharedString {
        match self {
            Self::WirelessOn => "icons/status_bar/wireless_on.svg",
            Self::WirelessOff => "icons/status_bar/wireless_off.svg",
            Self::WireleessWarning => "icons/status_bar/wireless_warning.svg",
            Self::WirelessHigh => "icons/status_bar/wireless_high.svg",
            Self::WirelessMedium => "icons/status_bar/wireless_medium.svg",
            Self::WirelessLow => "icons/status_bar/wireless_low.svg",
            Self::BluetoothOn => "icons/status_bar/bluetooth_on.svg",
            Self::BluetoothOff => "icons/status_bar/bluetooth_off.svg",
            Self::BluetoothConnected => "icons/status_bar/bluetooth_connected.svg",
            Self::BluetoothWarning => "icons/status_bar/bluetooth_warning.svg",
            Self::Battery10 => "icons/status_bar/battery_10.svg",
            Self::Battery20 => "icons/status_bar/battery_20.svg",
            Self::Battery30 => "icons/status_bar/battery_30.svg",
            Self::Battery40 => "icons/status_bar/battery_40.svg",
            Self::Battery50 => "icons/status_bar/battery_50.svg",
            Self::Battery60 => "icons/status_bar/battery_60.svg",
            Self::Battery70 => "icons/status_bar/battery_70.svg",
            Self::Battery80 => "icons/status_bar/battery_80.svg",
            Self::Battery90 => "icons/status_bar/battery_90.svg",
            Self::Battery100 => "icons/status_bar/battery_100.svg",
            Self::BatteryEmpty => "icons/status_bar/battery_empty.svg",
            Self::Battery10Charging => "icons/status_bar/battery_10_charging.svg",
            Self::Battery20Charging => "icons/status_bar/battery_20_charging.svg",
            Self::Battery30Charging => "icons/status_bar/battery_30_charging.svg",
            Self::Battery40Charging => "icons/status_bar/battery_40_charging.svg",
            Self::Battery50Charging => "icons/status_bar/battery_50_charging.svg",
            Self::Battery60Charging => "icons/status_bar/battery_60_charging.svg",
            Self::Battery70Charging => "icons/status_bar/battery_70_charging.svg",
            Self::Battery80Charging => "icons/status_bar/battery_80_charging.svg",
            Self::Battery90Charging => "icons/status_bar/battery_90_charging.svg",
            Self::Battery100Charging => "icons/status_bar/battery_100_charging.svg",
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
            // .when_some(self.size, |this, size| this.size(size))
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
