use crate::errors::{AppSwitcherError, AppSwitcherErrorCodes};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use std::{env, fs::File, path::PathBuf};
use anyhow::bail;

/// # App Switcher Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the app switcher, 
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppSwitcherSettings {
    pub app: AppSettings,
    pub window: WindowSettings,
    // Window Settings
    pub title: String,
    // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
}

impl Default for AppSwitcherSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("App Switcher"),
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
    pub id: Option<String>,
    // Process ID
    pub text_multithreading: bool,
    // Enable text multithreading
    pub antialiasing: bool,
    // Enable antialiasing
    pub try_opengles_first: bool,   // Enable using OpenGL ES before OpenGL (only for flow)
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: Some(String::from("app-switcher")),
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
    pub size: (u32, u32),
    // Size of the window
    pub position: (i32, i32),
    // Default position to start window
    pub min_size: Option<(u32, u32)>,
    // Minimum size the window can be resized to
    pub max_size: Option<(u32, u32)>,
    // Maximum size the window can be resized to
    pub visible: bool,
    // Sets visibility of the window
    pub resizable: bool,
    // Enables or disables resizing
    pub decorations: bool,
    // Enables or disables the title bar
    pub transparent: bool,
    // Enables transparency
    pub always_on_top: bool,
    // Forces window to be always on top
    pub icon_path: Option<String>,
}

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the app switcher.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub grid: Vec<String>, //Items that will in grid
}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomeIconPaths {
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CloseAllIconPaths {
    pub default: Option<String>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CloseIconPaths {
    pub default: Option<String>,
}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NoAppsIconPaths {
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CommonLowMediumHighPaths {
    pub low: Option<String>,
    pub medium: Option<String>,
    pub high: Option<String>,
}

/// # Modules
///
/// Options that will be visible in app switcher
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CpuModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomeModule {
    pub icon: HomeIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CloseAllModule {
    pub icon: CloseAllIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CloseModule {
    pub icon: CloseIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NoAppsModule {
    pub icon: NoAppsIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MemoryModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
    pub total: String,
}

/// # Modules
///
/// Options that will be visible in actions
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub cpu: CpuModule,
    pub memory: MemoryModule,
    pub home: HomeModule,
    pub close_all: CloseAllModule,
    pub close: CloseModule,
    pub no_apps: NoAppsModule,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: (1024, 768),
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
            grid: vec![],
        }
    }
}

impl Default for Modules {
    fn default() -> Self {
        Self {
            cpu: CpuModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "CPU".to_string(),
            },
            memory: MemoryModule {
                icon: CommonLowMediumHighPaths {
                    low:None,
                    medium:None,
                    high:None,
                },
                title: "Memory".to_string(),
                total: "".to_string(),
            },
            home: HomeModule { icon: HomeIconPaths { default: None }, title: "Home".to_string() },
            close_all: CloseAllModule { icon: CloseAllIconPaths { default: None }, title: "Close All".to_string() },
            close: CloseModule { icon: CloseIconPaths { default: None }, title: "Close".to_string() },
            no_apps: NoAppsModule { icon: NoAppsIconPaths { default :None }, title: "No Apps".to_string() },
        }
    }
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

/// # Reads Settings YML 
///
/// Reads the `settings.yml` and parsers to AppSwitcherSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<AppSwitcherSettings> {
    let mut file_path = PathBuf::from(std::env::var("MECHA_APP_SWITCHER_SETTINGS_PATH")
        .unwrap_or(String::from("settings.yml"))); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!(task = "read_settings", "settings file location - {:?}", file_path);

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::SettingsReadError,
                format!("cannot read the settings.yml in the path - {}", e.to_string()),
            ));
        }
    };

    // read and parse
    let config: AppSwitcherSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
