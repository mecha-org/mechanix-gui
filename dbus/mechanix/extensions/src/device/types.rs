use evdevil::{ Evdev, Bus };
use evdevil::event::{ EventType, Key, Switch };
use serde::{ Deserialize, Serialize };
use zvariant::{ Type };
use anyhow::Result;
use toml;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub enum DeviceType {
    Extension,
    VideoBus,
    PowerButton,
    SleepButton,
    LidSwitch,
    Unclassified,
}

impl DeviceType {
    pub fn get_device_type(evdev: &Evdev) -> Self {
        let unique_id = evdev
            .unique_id()
            .ok()
            .flatten()
            .unwrap_or_else(|| "Unknown".to_string());
        if is_extension(unique_id) {
            return Self::Extension;
        }
        // Get device name for additional context
        let name = evdev.name().unwrap_or_default().to_lowercase();

        // Check supported event types
        let supported_events = match evdev.supported_events() {
            Ok(events) => events,
            Err(_) => {
                return Self::Unclassified;
            }
        };

        // Video Bus devices
        if name.contains("video bus") || name.contains("video") {
            return Self::VideoBus;
        }

        // Power Button devices
        if name.contains("power") && name.contains("button") {
            return Self::PowerButton;
        }

        // Sleep Button devices
        if name.contains("sleep") && name.contains("button") {
            return Self::SleepButton;
        }

        // Lid Switch devices
        if name.contains("lid") && name.contains("switch") {
            return Self::LidSwitch;
        }

        // Check by supported keys and switches
        if supported_events.contains(EventType::KEY) {
            if let Ok(keys) = evdev.supported_keys() {
                // Power button typically supports KEY_POWER
                if keys.contains(Key::KEY_POWER) && !keys.contains(Key::KEY_A) {
                    return Self::PowerButton;
                }

                // Sleep button typically supports KEY_SLEEP or KEY_SUSPEND
                if keys.contains(Key::KEY_SLEEP) || keys.contains(Key::KEY_SUSPEND) {
                    return Self::SleepButton;
                }
            }
        }

        // Check switches for lid detection
        if supported_events.contains(EventType::SW) {
            if let Ok(switches) = evdev.supported_switches() {
                if switches.contains(Switch::LID) {
                    return Self::LidSwitch;
                }
            }
        }

        // Check physical location for more context
        if let Ok(Some(phys)) = evdev.phys() {
            let phys_lower = phys.to_lowercase();
            if phys_lower.contains("video") {
                return Self::VideoBus;
            }
        }

        // Default fallback
        Self::Unclassified
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum ConnectionType {
    Wired, // USB, PCI, I2C, SPI, etc.
    Wireless, // Bluetooth
    Virtual, // Virtual devices
    Legacy, // ISA, RS232, Gameport, etc.
    Unknown, // Unknown or unclassified bus types
}

impl ConnectionType {
    pub fn get_connection_type(evdev: &Evdev) -> Self {
        // Get the bus type from input_id
        let bus = match evdev.input_id() {
            Ok(input_id) => input_id.bus(),
            Err(_) => {
                return Self::Unknown;
            }
        };

        match bus {
            // Wired connections
            | Bus::USB
            | Bus::PCI
            | Bus::I2C
            | Bus::SPI
            | Bus::I8042
            | Bus::RMI
            | Bus::CEC
            | Bus::INTEL_ISHTP
            | Bus::AMD_SFH => Self::Wired,

            // Wireless connections
            Bus::BLUETOOTH => Self::Wireless,

            // Virtual devices
            Bus::VIRTUAL | Bus::HOST => Self::Virtual,

            // Legacy connections
            | Bus::ISA
            | Bus::ISAPNP
            | Bus::RS232
            | Bus::GAMEPORT
            | Bus::PARPORT
            | Bus::XTKBD
            | Bus::HIL
            | Bus::AMIGA
            | Bus::ADB
            | Bus::GSC
            | Bus::ATARI => Self::Legacy,

            // Unknown or unhandled bus types
            _ => Self::Unknown,
        }
    }
}

fn is_extension(id: String) -> bool {
    match fs::read_to_string("./config.toml") {
        Ok(raw) => {
            match toml::from_str::<ExtensionConfig>(&raw) {
                Ok(cfg) => {
                    let ids: Vec<String> = cfg.extensions
                        .into_iter()
                        .map(|ext| ext.unique_id)
                        .collect();
                    ids.contains(&id)
                }
                Err(e) => {
                    println!("Error parsing TOML: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            false
        }
    }
}

#[derive(Debug, Deserialize)]
struct Extension {
    unique_id: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExtensionConfig {
    extensions: Vec<Extension>,
}