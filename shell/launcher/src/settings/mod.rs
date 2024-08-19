use crate::constants::*;
use crate::errors::{LauncherError, LauncherErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Launcher Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the launcher,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct  LauncherSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub fonts: CustomFonts
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Launcher"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            fonts: CustomFonts::default(),
        }
    }
}

/// # App Settings
///
/// Struct part of settings.yml to control the application
/// behavior, includes optimizations and defaults
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppSettings {
    pub id: Option<String>,        // Process ID
    pub text_multithreading: bool, // Enable text multithreading
    pub antialiasing: bool,        // Enable antialiasing
    pub try_opengles_first: bool,  // Enable using OpenGL ES before OpenGL (only for flow)
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: Some(String::from("Launcher")),
            text_multithreading: false,
            antialiasing: false,
            try_opengles_first: true,
        }
    }
}

/// # Window Settings
///
/// Part of the settings.yml to control the behavior of
/// the application window
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WindowSettings {
    pub size: (i32, i32),             // Size of the window
    pub position: (i32, i32),         // Default position to start window
    pub min_size: Option<(u32, u32)>, // Minimum size the window can be resized to
    pub max_size: Option<(u32, u32)>, // Maximum size the window can be resized to
    pub visible: bool,                // Sets visibility of the window
    pub resizable: bool,              // Enables or disables resizing
    pub decorations: bool,            // Enables or disables the title bar
    pub transparent: bool,            // Enables transparency
    pub always_on_top: bool,          // Forces window to be always on top
    pub icon_path: Option<String>,
}

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the launcher.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of launcher
    pub center: Vec<String>, //Items that will in center of launcher
    pub right: Vec<String>,  //Items that will in right side of launcher
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct App {
    pub app_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub run_command: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundModule {
    pub icon: BackgroundIconPath,
}
impl Default for BackgroundModule {
    fn default() -> Self {
        Self {
            icon: BackgroundIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct RotationModule {
    pub icon: RotationIconPaths,
    pub title: String,
}
impl Default for RotationModule {
    fn default() -> Self {
        RotationModule {
            icon: RotationIconPaths::default(),
            title: "Auto Rotate".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct RotationIconPaths {
    pub portrait: String,
    pub landscape: String,
}
impl Default for RotationIconPaths {
    fn default() -> Self {
        RotationIconPaths {
            portrait: ROTATION_PORTRAIT.to_owned(),
            landscape: ROTATION_LANDSCAPE.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PowerModule {
    pub icon: PowerIconPath,
}
impl Default for PowerModule {
    fn default() -> Self {
        Self {
            icon: PowerIconPath::default(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PowerIconPath {
    pub default: String,
}
impl Default for PowerIconPath {
    fn default() -> Self {
        PowerIconPath {
            default: POWER_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SearchModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClearModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DefaultIconPaths {
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundIconPath {
    pub default: String,
}
impl Default for BackgroundIconPath {
    fn default() -> Self {
        Self {
            default: BACKGROUND_IMAGE.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in launcher
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub clock: Clock,
    pub bluetooth: Bluetooth,
    pub wireless: Wireless,
    pub battery: Battery,
    pub apps: Vec<App>,
    pub background: BackgroundModule,
    pub rotation: RotationModule,
    pub power: PowerModule,
}



impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: (480, 440),
            position: (0, 0),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon_path: None,
        }
    }
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            left: vec![],
            center: vec![],
            right: vec![],
        }
    }
}

/// Clock module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Clock {
    pub format: String,
}

/// Bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bluetooth {
    #[serde(default)]
    pub icon: BluetoothIconPaths,
}
impl Default for Bluetooth {
    fn default() ->  Self {
        Bluetooth {
        icon: BluetoothIconPaths::default(),
    }}
}

/// Wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Wireless {
    #[serde(default)]
    pub icon: WirelessIconPaths,
}
impl Default for Wireless {
    fn default() ->  Self {
        Wireless {
        icon: WirelessIconPaths::default(),
    }}
}


/// Battery module 
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Battery {
    #[serde(default)]
    pub icon: BatteryIconPaths,
    #[serde(default)] 
    pub charging_icon: ChargingBatteryIconPaths,
}
impl Default for Battery {
    fn default() ->  Self {
        Battery {
        icon: BatteryIconPaths::default(),
        charging_icon: ChargingBatteryIconPaths::default(),
    }}
}


/// Icon paths for bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct BluetoothIconPaths {
    pub on: String,
    pub off: String,
    pub connected: String,
    pub not_found: String,
}
impl Default for BluetoothIconPaths {
    fn default() -> Self {
        BluetoothIconPaths {
            off: BLUETOOTH_OFF.to_owned(),
            on: BLUETOOTH_ON.to_owned(),
            connected: BLUETOOTH_CONNECTED.to_owned(),
            not_found: BLUETOOTH_NOT_FOUND.to_owned(),
        }
    
    }
}

/// Icon paths for wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct WirelessIconPaths {
    pub off: String,
    pub on: String,
    pub low: String,
    pub weak: String,
    pub good: String,
    pub strong: String,
    pub not_found: String,
}
impl Default for WirelessIconPaths {
    fn default() -> Self {
        WirelessIconPaths {
            off: WIRELESS_OFF.to_owned(),
            on: WIRELESS_ON.to_owned(),
            low: WIRELESS_LOW.to_owned(),
            weak: WIRELESS_WEAK.to_owned(),
            good: WIRELESS_GOOD.to_owned(),
            strong: WIRELESS_STRONG.to_owned(),
            not_found: WIRELESS_NOT_FOUND.to_owned(),
        }
    
    }
}

// /// Icon paths for battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct BatteryIconPaths {
    pub level_100: String,
    pub level_90: String,
    pub level_80: String,
    pub level_70: String,
    pub level_60: String,
    pub level_50: String,
    pub level_40: String,
    pub level_30: String,
    pub level_20: String,
    pub level_10: String,
    pub level_0: String,
    pub not_found: String,
}

impl Default for BatteryIconPaths {
    fn default() ->  Self {
        BatteryIconPaths {
            level_100: BATTERY_LEVEL_100.to_owned(),
            level_90: BATTERY_LEVEL_90.to_owned(),
            level_80: BATTERY_LEVEL_80.to_owned(),
            level_70: BATTERY_LEVEL_70.to_owned(),
            level_60: BATTERY_LEVEL_60.to_owned(),
            level_50: BATTERY_LEVEL_50.to_owned(),
            level_40: BATTERY_LEVEL_40.to_owned(),
            level_30: BATTERY_LEVEL_30.to_owned(),
            level_20: BATTERY_LEVEL_20.to_owned(),
            level_10: BATTERY_LEVEL_10.to_owned(),
            level_0: BATTERY_LEVEL_0.to_owned(),
            not_found: BATTERY_NOT_FOUND.to_owned(),
        }
    }
}



#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct ChargingBatteryIconPaths {
    pub level_100: String,
    pub level_90: String,
    pub level_80: String,
    pub level_70: String,
    pub level_60: String,
    pub level_50: String,
    pub level_40: String,
    pub level_30: String,
    pub level_20: String,
    pub level_10: String,
    pub level_0: String,
    pub not_found: String,
}

impl Default for ChargingBatteryIconPaths {
    fn default() ->  Self {
        ChargingBatteryIconPaths {
            level_100: CHARGING_BATTERY_LEVEL_100.to_owned(),
            level_90: CHARGING_BATTERY_LEVEL_90.to_owned(),
            level_80: CHARGING_BATTERY_LEVEL_80.to_owned(),
            level_70: CHARGING_BATTERY_LEVEL_70.to_owned(),
            level_60: CHARGING_BATTERY_LEVEL_60.to_owned(),
            level_50: CHARGING_BATTERY_LEVEL_50.to_owned(),
            level_40: CHARGING_BATTERY_LEVEL_40.to_owned(),
            level_30: CHARGING_BATTERY_LEVEL_30.to_owned(),
            level_20: CHARGING_BATTERY_LEVEL_20.to_owned(),
            level_10: CHARGING_BATTERY_LEVEL_10.to_owned(),
            level_0: CHARGING_BATTERY_LEVEL_0.to_owned(),
            not_found: BATTERY_NOT_FOUND.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CustomFonts {
    pub paths: Vec<String>
}
impl Default for CustomFonts {
    fn default() -> Self {
        Self { paths: vec![] }
    }
}



impl Default for Modules {
    fn default() -> Self {

        Self {
            apps: vec![
                App{
                    app_id: "mecha-connect".to_string(),
                    name: "Mecha Connect".to_string(),
                    icon: Some(MECHA_CONNECT_ICON.to_owned()),
                    run_command:  [
                        "sh",
                        "-c",
                        "MECHA_CONNECT_APP_SETTINGS_PATH=/etc/mecha/connect/settings-demo.yml mecha-connect",
                      ].map(String::from).to_vec(),
                },
                App{
                    app_id: "mecha-llama".to_string(),
                    name: "Mecha LLama".to_string(),
                    icon: Some(MECHA_LLAMA_ICON.to_owned()),
                    run_command: [
                        "sh", 
                        "-c",
                        "chromium --user-agent='Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.43 Mobile Safari/537.36' --ozone-platform=wayland https://mecha-voice-ai-demo.vercel.app/"
                      ].map(String::from).to_vec(),
                },
                App{
                    app_id: "mecha-vision".to_string(),
                    name: "Mecha Vision".to_string(),
                    icon: Some(MECHA_VISION_ICON.to_owned()),
                    run_command: [
                        "sh", 
                        "-c",
                        "LD_LIBRARY_PATH=/home/mecha/.pipeless /home/mecha/.pipeless/pipeless add stream --input-uri 'v4l2' --output-uri 'screen' --frame-path 'cats'"
                      ].map(String::from).to_vec(),
                },
                App{
                    app_id: "mecha-terminal".to_string(),
                    name: "Mecha Terminal".to_string(),
                    icon: Some(MECHA_TERMINAL_ICON.to_owned()),
                    run_command: [
                        "sh", 
                        "-c",
                        "alacritty"
                      ].map(String::from).to_vec(),
                },
                App{
                    app_id: "mecha-gaming".to_string(),
                    name: "Mecha Gaming".to_string(),
                    icon: Some(MECHA_GAMING_ICON.to_owned()),
                    run_command: [
                        "sh", 
                        "-c",
                        "chromium --user-agent='Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.43 Mobile Safari/537.36' --ozone-platform=wayland https://guccigrip.gucci.com/"
                      ].map(String::from).to_vec(),
                },
            ],
            background: BackgroundModule::default(),
            clock: Clock {
                format: "%I:%M %p".to_string(),
            },
            bluetooth: Bluetooth {
                icon: BluetoothIconPaths::default(),
            },
            wireless: Wireless {
                icon: WirelessIconPaths::default(),
            },
            battery: Battery {
                icon: BatteryIconPaths::default(),
                charging_icon: ChargingBatteryIconPaths::default(),
            },
            rotation: RotationModule { icon: RotationIconPaths::default(), title: "".to_string() },
            power: PowerModule { icon: PowerIconPath::default() }
        
        }
    }
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

fn is_valid_file(path: &str) -> Option<PathBuf> {
    let path_buf = PathBuf::from(path);
    println!(
        "CHECKING PATH {} EXIST ===>  {:?}",
        path,
        path_buf.is_file()
    );
    if path_buf.is_file() {
        Some(path_buf)
    } else {
        None
    }
}

fn find_config_path() -> Option<PathBuf> {
    // from env
    if let Ok(env_path) = std::env::var("MECHA_LAUNCHER_SETTINGS_PATH") {
        if let Some(path) = is_valid_file(&env_path) {
            return Some(path);
        }
    }

    // read from args
    if let Some(arg) = read_settings_path_from_args() {
        if let Some(file_path_in_args) = is_valid_file(&arg) {
            return Some(PathBuf::from(file_path_in_args));
        }
    }

    // read from local settings
    if let settings_path = String::from("settings.yml") {
        if let Some(path) = is_valid_file(&settings_path) {
            return Some(path);
        }
    }

    // home config dir
    if let Some(home_dir) = dirs::home_dir() {
        let mut path = home_dir;
        path.push(&format!("{}{}", HOME_DIR_CONFIG_PATH, BASE_SETTINGS_PATH)); // Replace with your actual path
        if let Some(path) = is_valid_file(path.to_str().unwrap()) {
            return Some(path);
        }
    }

    // default usr dir
    let default_path = format!("{}{}", USR_SHARE_PATH, BASE_SETTINGS_PATH);
    is_valid_file(&default_path)
}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to LauncherSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<LauncherSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(LauncherError::new(
            LauncherErrorCodes::SettingsReadError,
            format!("settings.yml path not found",),
        ));
    }

    info!(
        task = "read_settings",
        "settings file location - {:?}", file_path
    );

    // open file
    let settings_file_handle = match File::open(file_path.unwrap()) {
        Ok(file) => file,
        Err(e) => {
            bail!(LauncherError::new(
                LauncherErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: LauncherSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(LauncherError::new(
                LauncherErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
