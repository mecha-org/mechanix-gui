use crate::constants::{
    BACKGROUND_IMAGE, BASE_SETTINGS_PATH, HOME_DIR_CONFIG_PATH, MECHA_CONNECT_ICON, MECHA_GAMING_ICON, MECHA_LLAMA_ICON, MECHA_TERMINAL_ICON, MECHA_VISION_ICON, USR_SHARE_PATH
};
use crate::errors::{HomescreenError, HomescreenErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Homescreen Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the homescreen,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomescreenSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
}

impl Default for HomescreenSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("home screen"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
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
            id: Some(String::from("homescreen")),
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
/// the layout of options in the home screen.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of home screen
    pub center: Vec<String>, //Items that will in center of home screen
    pub right: Vec<String>,  //Items that will in right side of home screen
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
/// Options that will be visible in home screen
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub apps: Vec<App>,
    pub background: BackgroundModule,
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
    println!("CHECKING PATH {} EXIST ===>  {:?}", path, path_buf.is_file());
    if path_buf.is_file() {
        Some(path_buf)
    } else {
        None
    }
}

fn find_config_path() -> Option<PathBuf> {

    // from env 
    if let Ok(env_path) = std::env::var("MECHA_HOMESCREEN_SETTINGS_PATH") {
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
    if let Some(home_dir) = dirs::home_dir(){
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
/// Reads the `settings.yml` and parsers to HomescreenSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<HomescreenSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(HomescreenError::new(
            HomescreenErrorCodes::SettingsReadError,
            format!(
                "settings.yml path not found",
            ),
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
            bail!(HomescreenError::new(
                HomescreenErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: HomescreenSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(HomescreenError::new(
                HomescreenErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
