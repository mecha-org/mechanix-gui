// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Icon {
//     Moon = 0xe900,
//     Bulb = 0xe901,
//     AirplaneMode = 0xe902,
//     AutoRotation = 0xe903,
//     Monitor = 0xe904, // external display
//     ScreenRecord = 0xe905,
//     Camera = 0xe908,
//     Terminal = 0xe910,
//     Mic = 0xe911, // Voice_Record
//     Calc = 0xe912,
//     Brightness = 0xe913, // sun
//     VolumeOn = 0xe914,
//     VolumeOff = 0xe915,
//     Usb = 0xe916,
//     Headphones = 0xe918,

//     /// Wifi icons
//     WifiConnectedStrong = 0xe906,
//     WifiOff = 0xe920,
//     WifiMedium = 0xe921,
//     WifiLow = 0xe926,
//     // WifiAnimated,
//     WifiNoInternet = 0xe924,
//     WifiOnNotConnected = 0xe925,

//     /// Bluetooth icons
//     Bluetooth = 0xe907,
//     BluetoothConnected = 0xe919,

//     /// Battery icons
//     Battery = 0xe909,
//     BatteryCharging = 0xe90a,
//     BatteryLow = 0xe932,
//     BatteryCriticallyLow = 0xe92f,
//     BatteryChargedComplete = 0xe90c,
//     BatteryNoBattery = 0xe90b,

//     /// Mobile Network icons
//     SignalBarsFull = 0xe917,
//     // SignalBarsMedium ,
//     // SignalBarsLow ,
//     // NoSim ,
//     // NoSignal ,
//     Settings = 0xe933,
//     Power = 0xe934,
//     Edit = 0xe935,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    //Battery
    BatteryWarning = 0xe0c8,

    BatteryEmpty = 0xe0be,
    BatteryFull = 0xe0c0,
    BatteryHigh = 0xe0c2,
    BatteryLow = 0xe0c4,
    BatteryMedium = 0xe0c6,

    BatteryEmptyCharging = 0xe905,
    BatteryFullCharging = 0xe901,
    BatteryHighCharging = 0xe902,
    BatteryLowCharging = 0xe904,
    BatteryMediumCharging = 0xe903,

    //Bluetooth
    BluetoothNone = 0xe0da,
    BluetoothOff = 0xe0de,
    BluetoothConnected = 0xe0dc,
    BluetoothWarning = 0xe0e0,

    //Notification
    Notification = 0xe900,

    //Wireless
    WirelessHigh = 0xe4ea,
    WirelessLow = 0xe4ec,
    WirelessMedium = 0xe4ee,
    WirelessNone = 0xe4f0,
    WirelessOff = 0xe4f2,
    WirelessWarning = 0xe4f4,
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
