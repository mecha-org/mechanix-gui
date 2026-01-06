use std::path::PathBuf;

use commons::prelude::*;
use gpui::layer_shell::{Anchor, Layer};
use gpui::*;
use serde::Deserialize;
use toml::{Table, Value};

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct Settings {
    #[serde(default)]
    pub status_bar: StatusBarSettings,
    #[serde(default)]
    pub settings_drawer: SettingsDrawerSettings,
    #[serde(default)]
    pub running_apps: RunningAppsSettings,
    #[serde(default)]
    pub universal_search: UniversalSearchSettings,
    #[serde(default)]
    pub notifications: NotificationSettings,
    #[serde(default)]
    pub app_drawer: AppDrawerSettings,
    #[serde(default)]
    pub homescreen: HomescreenSettings,
    #[serde(default)]
    pub power_options: PowerOptionsSettings,
    #[serde(default)]
    pub volume_slider: VolumeSliderSettings,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LayerShellSettings {
    #[serde(default)]
    pub size: Size<Pixels>,
    #[serde(default)]
    pub layer: Layer,
    #[serde(default)]
    pub anchor: Anchor,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub exclusive_zone: Pixels,
}

impl Default for LayerShellSettings {
    fn default() -> Self {
        Self {
            size: Size::new(px(540.0), px(620.0)),
            layer: Layer::Top,
            anchor: Anchor::all(),
            namespace: "".into(),
            exclusive_zone: px(0.0),
        }
    }
}

/// Status bar settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct StatusBarSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
}

impl Default for StatusBarSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Overlay,
                anchor: Anchor::RIGHT | Anchor::LEFT | Anchor::TOP,
                namespace: "mechanix.status.bar".into(),
                exclusive_zone: px(0.0),
                size: Size::new(px(540.0), px(300.0)),
            },
        }
    }
}

/// Running apps settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RunningAppsSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
    #[serde(default)]
    pub navbar_size: Size<Pixels>,
}

impl Default for RunningAppsSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Top,
                anchor: Anchor::TOP,
                namespace: "mechanix.running.apps".into(),
                exclusive_zone: px(-1.0),
                size: Size::new(px(540.0), px(300.0)),
            },
            navbar_size: Size::new(px(199.22), px(28.5)),
        }
    }
}

/// Settings drawer settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SettingsDrawerSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
    #[serde(default)]
    pub navbar_size: Size<Pixels>,
}

impl Default for SettingsDrawerSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Top,
                anchor: Anchor::BOTTOM,
                namespace: "mechanix.settings.drawer".into(),
                exclusive_zone: px(-1.0),
                size: Size::new(px(540.0), px(620.0)),
            },
            navbar_size: Size::new(px(199.22), px(28.5)),
        }
    }
}

/// Universal search settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UniversalSearchSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
    #[serde(default)]
    pub navbar_size: Size<Pixels>,
}

impl Default for UniversalSearchSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Top,
                anchor: Anchor::BOTTOM,
                namespace: "mechanix.universal.search".into(),
                exclusive_zone: px(-1.0),
                size: Size::new(px(540.0), px(620.0)),
            },
            navbar_size: Size::new(px(199.22), px(28.5)),
        }
    }
}

/// Notification settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct NotificationSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
    #[serde(default)]
    pub navbar_size: Size<Pixels>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Top,
                anchor: Anchor::BOTTOM,
                namespace: "mechanix.notifications".into(),
                exclusive_zone: px(-1.0),
                size: Size::new(px(540.0), px(620.0)),
            },
            navbar_size: Size::new(px(199.22), px(28.5)),
        }
    }
}

/// App drawer settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AppDrawerSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
}

impl Default for AppDrawerSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Top,
                anchor: Anchor::TOP,
                namespace: "mechanix.app.drawer".into(),
                exclusive_zone: px(0.0),
                size: Size::new(px(540.0), px(620.0)),
            },
        }
    }
}

/// Homescreen settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct HomescreenSettings {
    #[serde(default)]
    pub status_bar_size: Size<Pixels>,

    #[serde(default)]
    pub layer_shell: LayerShellSettings,
}

impl Default for HomescreenSettings {
    fn default() -> Self {
        Self {
            status_bar_size: Size::new(px(540.0), px(36.0)),
            layer_shell: LayerShellSettings {
                layer: Layer::Bottom,
                anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT | Anchor::BOTTOM,
                namespace: "mechanix.homescreen".into(),
                exclusive_zone: px(0.0),
                size: Size::new(px(540.0), px(620.0)),
            },
        }
    }
}

/// Power options settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PowerOptionsSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
}

impl Default for PowerOptionsSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Overlay,
                anchor: Anchor::TOP,
                namespace: "mechanix.power.options".into(),
                exclusive_zone: px(0.0),
                size: Size::new(px(540.0), px(620.0)),
            },
        }
    }
}

/// Volume slider settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct VolumeSliderSettings {
    #[serde(default)]
    pub layer_shell: LayerShellSettings,
    #[serde(default)]
    pub min_volume_level: f32,
    #[serde(default)]
    pub max_volume_level: f32,
}

impl Default for VolumeSliderSettings {
    fn default() -> Self {
        Self {
            layer_shell: LayerShellSettings {
                layer: Layer::Overlay,
                anchor: Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                namespace: "mechanix.hardware_buttons.slider".into(),
                exclusive_zone: px(0.0),
                size: Size::new(px(500.0), px(60.0)),
            },
            min_volume_level: 0.0,
            max_volume_level: 100.0,
        }
    }
}

pub fn config_paths_for(file_name: &str) -> Vec<PathBuf> {
    let mut config_paths = Vec::new();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        if let Ok(dev_asset_dir) = PathBuf::from(manifest_dir)
            .join(format!("../../../assets/{}", file_name))
            .canonicalize()
        {
            config_paths.push(dev_asset_dir);
        };
    } else {
        println!("CARGO_MANIFEST_DIR not set");
    };

    config_paths.push(PathBuf::from(format!(
        "/usr/share/mechanix/launcher/assets/{}",
        file_name
    )));
    config_paths.push(PathBuf::from(format!(
        "/etc/mechanix/launcher/assets/{}",
        file_name
    )));

    if let Some(home_dir) = dirs::home_dir() {
        config_paths.push(home_dir.join(format!(".config/mechanix/launcher/assets/{}", file_name)));
    }

    config_paths
}

pub fn load_settings<T>(config_paths: Vec<PathBuf>) -> T
where
    T: for<'de> serde::de::Deserialize<'de> + Default,
{
    let mut merged: Value = Table::new().into();

    for raw_path in config_paths {
        let content = match std::fs::read_to_string(&raw_path) {
            Ok(c) => c,
            Err(e) => {
                // println!("Could not read config file {}: {}", raw_path.display(), e);
                continue;
            }
        };

        if let Err(e) = toml::from_str::<T>(&content) {
            // println!("Deserialization failed for {}: {}", raw_path.display(), e);
            continue;
        }

        let table: Table = match content.parse() {
            Ok(t) => t,
            Err(e) => {
                // println!("Could not parse TOML from {}: {}", raw_path.display(), e);
                continue;
            }
        };

        merged = match merge(merged.clone().into(), table.into()) {
            Ok(v) => v,
            Err(e) => {
                // println!("Merge failed for {}: {:?}", raw_path.display(), e);
                continue;
            }
        };
    }

    if merged.as_array().iter().len() >= 0 {
        merged.clone().try_into().unwrap_or_default()
    } else {
        T::default()
    }
}
