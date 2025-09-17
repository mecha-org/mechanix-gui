use evdevil::{ Evdev, Bus };
use evdevil::event::{ EventType, Key, Switch };
use serde::{ Deserialize, Serialize };
use zvariant::{ Type };
use anyhow::Result;
use toml;
use std::fs;
use std::env;
use std::path::PathBuf;

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
    pub fn get_device_type(evdev: &Evdev) -> (Self, String) {
        // Default key for non-extension devices
        let default_key = "KEY_RESERVED".to_string();

        let unique_id = evdev
            .unique_id()
            .ok()
            .flatten()
            .unwrap_or_else(|| "Unknown".to_string());
        
        if let Some(connection_key) = is_extension(unique_id) {
            return (Self::Extension, connection_key);
        } 

        // Get device name for additional context
        let name = evdev.name().unwrap_or_default().to_lowercase();

        // Check supported event types
        let supported_events = match evdev.supported_events() {
            Ok(events) => events,
            Err(_) => {
                return (Self::Unclassified, default_key);
            }
        };

        // Video Bus devices
        if name.contains("video bus") || name.contains("video") {
            return (Self::VideoBus, default_key);
        }

        // Power Button devices
        if name.contains("power") && name.contains("button") {
            return (Self::PowerButton, default_key);
        }

        // Sleep Button devices
        if name.contains("sleep") && name.contains("button") {
            return (Self::SleepButton, default_key);
        }

        // Lid Switch devices
        if name.contains("lid") && name.contains("switch") {
            return (Self::LidSwitch, default_key);
        }

        // Check by supported keys and switches
        if supported_events.contains(EventType::KEY) {
            if let Ok(keys) = evdev.supported_keys() {
                // Power button typically supports KEY_POWER
                if keys.contains(Key::KEY_POWER) && !keys.contains(Key::KEY_A) {
                    return (Self::PowerButton, default_key);
                }

                // Sleep button typically supports KEY_SLEEP or KEY_SUSPEND
                if keys.contains(Key::KEY_SLEEP) || keys.contains(Key::KEY_SUSPEND) {
                    return (Self::SleepButton, default_key);
                }
            }
        }

        // Check switches for lid detection
        if supported_events.contains(EventType::SW) {
            if let Ok(switches) = evdev.supported_switches() {
                if switches.contains(Switch::LID) {
                    return (Self::LidSwitch, default_key);
                }
            }
        }

        // Check physical location for more context
        if let Ok(Some(phys)) = evdev.phys() {
            let phys_lower = phys.to_lowercase();
            if phys_lower.contains("video") {
                return (Self::VideoBus, default_key);
            }
        }

        // Default fallback
        (Self::Unclassified, default_key)
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

fn is_extension(id: String) -> Option<String> {
    // Get config path from environment variable or use default
    let config_path = env::var("MECHANIX_EXTENSION_CONFIG_PATH")
        .unwrap_or_else(|_| {
            let mut path = PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()));
            path.push(".config/mechanix/extensions/config.toml");
            path.to_string_lossy().to_string()
        });

    match fs::read_to_string(&config_path) {
        Ok(raw) => {
            match toml::from_str::<ExtensionConfig>(&raw) {
                Ok(cfg) => {
                    for ext in cfg.extensions {
                        if ext.unique_id == id {
                            return Some(ext.connection_key);
                        }
                    }
                    None
                }
                Err(e) => {
                    println!("Error parsing TOML: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error reading config file '{}': {}", config_path, e);
            None
        }
    }
}

#[derive(Debug, Deserialize)]
struct Extension {
    unique_id: String,
    name: Option<String>,
    connection_key: String,
}

#[derive(Debug, Deserialize)]
struct ExtensionConfig {
    extensions: Vec<Extension>,
}
