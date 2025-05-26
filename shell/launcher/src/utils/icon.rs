#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Moon = 0xe900,
    Bulb = 0xe901,
    AirplaneMode = 0xe902,
    AutoRotation = 0xe903,
    Monitor = 0xe904, // external display
    ScreenRecord = 0xe905,
    Camera = 0xe908,
    Terminal = 0xe910,
    Mic = 0xe911, // Voice_Record
    Calc = 0xe912,
    Brightness = 0xe913, // sun
    VolumeOn = 0xe914,
    VolumeOff = 0xe915,
    Usb = 0xe916,
    Headphones = 0xe918,

    /// Wifi icons
    WifiConnectedStrong = 0xe906,
    WifiOff = 0xe920,
    WifiMedium = 0xe921,
    WifiLow = 0xe926,
    // WifiAnimated,
    WifiNoInternet = 0xe924,
    WifiOnNotConnected = 0xe925,

    /// Bluetooth icons
    Bluetooth = 0xe907,
    BluetoothConnected = 0xe919,

    /// Battery icons
    Battery = 0xe909,
    BatteryCharging = 0xe90a,
    BatteryLow = 0xe932,
    BatteryCriticallyLow = 0xe92f,
    BatteryChargedComplete = 0xe90c,
    BatteryNoBattery = 0xe90b,

    /// Mobile Network icons
    SignalBarsFull = 0xe917,
    // SignalBarsMedium ,
    // SignalBarsLow ,
    // NoSim ,
    // NoSignal ,
}

impl Icon {
    /// Converts the icon to its UTF-8 string representation.
    pub fn to_string(&self) -> String {
        let code_point = *self as u32;

        match std::char::from_u32(code_point) {
            Some(c) => String::from(c),
            None => String::new(),
        }
    }
}

impl Into<String> for Icon {
    fn into(self) -> String {
        self.to_string()
    }
}
