use battery::Manager;
pub struct Power {
    battery: battery::Battery,
}

#[derive(Debug)]
pub struct BatteryInfo<'a> {
    pub vendor: Option<&'a str>,
    pub model: Option<&'a str>,
    pub serial_number: Option<&'a str>,
    pub technology: String,
    pub state: String,
}

impl Power {
    pub fn new() -> Self {
        let manager = Manager::new().unwrap();
        let battery = manager.batteries().unwrap().next().unwrap().unwrap();

        Power { battery: battery }
    }

    pub fn info(&self) -> String {
        let info = format!("Battery: {:?}", self.battery);
        info
    }

    pub fn get_battery_status(&self) -> String {
        let status = match self.battery.state() {
            battery::State::Charging => "Charging",
            battery::State::Discharging => "Discharging",
            battery::State::Empty => "Empty",
            battery::State::Full => "Full",
            battery::State::Unknown => "Unknown",
            _ => "Unknown",
        };

        status.to_string()
    }

    pub fn get_battery_info(&self) -> BatteryInfo {
        BatteryInfo {
            vendor: self.battery.vendor(),
            model: self.battery.model(),
            serial_number: self.battery.serial_number(),
            technology: self.battery.technology().to_string(),
            state: self.battery.state().to_string(),
        }
    }
}
