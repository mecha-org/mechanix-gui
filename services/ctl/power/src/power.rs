use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use system_shutdown::{hibernate, logout, reboot, shutdown};

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

    pub fn get_battery_percentage(&self) -> f32 {
        self.battery.state_of_charge().value * 100.0
    }

    // we need to get all available governors form  reading this file /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors and list it
    pub fn get_available_governors(&self) -> Result<Vec<String>, std::io::Error> {
        let governor_path =
            Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors");

        let mut governors = Vec::new();
        let file = File::open(governor_path)?;
        let mut reader = BufReader::new(file);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        for governor in buffer.split_whitespace() {
            governors.push(governor.to_string());
        }

        Ok(governors)
    }

    //get current governor by reading this file /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
    pub fn get_current_cpu_governor(&self) -> Result<String, std::io::Error> {
        let governor_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor");

        let mut buffer = String::new();
        let file = File::open(governor_path)?;
        let mut reader = BufReader::new(file);

        reader.read_to_string(&mut buffer)?;
        buffer.trim().to_string(); // Remove leading/trailing whitespace

        Ok(buffer)
    }

    // we need to set the cpu governor by writing to this file /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
    pub fn set_cpu_governor(&self, governor: &str) -> Result<(), std::io::Error> {
        let governor_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor");
        let mut file = File::create(governor_path)?;
        file.write_all(governor.as_bytes())?;

        Ok(())
    }

    //get cpu frequency by reading this file /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
    pub fn get_cpu_frequency() -> Result<String, std::io::Error> {
        let frequency_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq");

        let mut buffer = String::new();
        let file = File::open(frequency_path)?;
        let mut reader = BufReader::new(file);

        reader.read_to_string(&mut buffer)?;
        buffer.trim().to_string(); // Remove leading/trailing whitespace

        Ok(buffer)
    }

    //power off the system
    pub fn power_off(&self) -> Result<(), std::io::Error> {
        match shutdown() {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to power off",
            )),
        }
    }
    //reboot the system
    pub fn reboot(&self) -> Result<(), std::io::Error> {
        match reboot() {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to reboot",
            )),
        }
    }

    //logout the current session
    pub fn session_logout(&self) -> Result<(), std::io::Error> {
        match logout() {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to logout",
            )),
        }
    }

    //suspend the system
    pub fn suspend(&self) -> Result<(), std::io::Error> {
        match hibernate() {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to suspend",
            )),
        }
    }
}
