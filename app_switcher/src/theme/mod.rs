use crate::errors::{AppSwitcherError, AppSwitcherErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Theme Settings
///
/// Struct representing the theme.yml configuration file,
/// this file lets you control the appearance and theme
/// of the launcher
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct AppSwitcherTheme {
    pub font: FontSettings,             // Font Settings
    pub colors: ColorSettings,          // Color Settings
    pub font_size: FontSizeSettings,    // Font Size Settings
    pub background: BackgroundSettings, // Background Settings
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ThemeConfigs {
    pub theme: AppSwitcherTheme, // Theme configs
}

/// # Font Settings
///
/// Declares all the fonts needed for the launcher with their
/// paths (relative to the binary)
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct FontSettings {
    pub heading: Option<Font>,
    pub default: Option<Font>,
}

//// # Font
////
//// Corresponds to a single font, and its path
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Font {
    pub name: Option<String>,
}

/// # Background Settings
///
/// Declares the background configuration
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FontSizeSettings {
    h0: Option<f32>,
    h1: Option<f32>,
    h2: Option<f32>,
    h3: Option<f32>,
    h4: Option<f32>,
    h5: Option<f32>,
    h6: Option<f32>,
    default: Option<f32>,
    sm: Option<f32>,
    xs: Option<f32>,
}

/// # Color Settings
///
/// Declares the color configuration
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ColorSettings {
    text: String,
    primary: String,
    accent: String,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            primary: "".to_string(),
            accent: "".to_string(),
        }
    }
}

impl Default for FontSizeSettings {
    fn default() -> Self {
        Self {
            h0: Some(40.0),
            h1: Some(32.0),
            h2: Some(26.0),
            h3: Some(22.0),
            h4: Some(20.0),
            h5: Some(18.0),
            h6: Some(16.0),
            default: Some(14.0),
            sm: Some(12.0),
            xs: Some(11.),
        }
    }
}

/// # Background Settings
///
/// Declares all the font sizes needed by
/// the application
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BackgroundSettings {
    pub default: Option<Background>,
}

impl Default for BackgroundSettings {
    fn default() -> Self {
        Self {
            default: Some(Background::default()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Background {
    pub color: (u8, u8, u8, f32),
    pub image: Option<String>,
    pub fill: Option<BackgroundFillType>,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            color: (0, 0, 0, 1.0),
            image: None,
            fill: Some(BackgroundFillType::Cover),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub enum BackgroundFillType {
    #[default]
    Centered,
    Stretch,
    Cover,
}

/// # Reads Theme path from arg
///
/// Reads the `-t` or `--theme` argument for the path
pub fn read_theme_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-t" || args[1] == "--theme") {
        debug!("using theme path from argument - {}", args[2]);
        return Some(args[2].clone());
    }
    None
}

/// # Reads Theme YML
///
/// Reads the `theme.yml` and parsers to AppSwitcherTheme
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_theme_yml() -> Result<AppSwitcherTheme> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_APP_DOCK_THEME_PATH").unwrap_or(String::from("theme.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_theme_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!("theme file location - {:?}", file_path);

    // open file
    let theme_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::ThemeReadError,
                format!("cannot read the theme.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let config: ThemeConfigs = match serde_yaml::from_reader(theme_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::ThemeParseError,
                format!("error parsing the theme.yml - {}", e),
            ));
        }
    };

    Ok(config.theme)
}
