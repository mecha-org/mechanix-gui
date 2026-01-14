use std::path::PathBuf;

use commons::prelude::*;
use serde::Deserialize;
use toml::{Table, Value};

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct Icons {
    #[serde(default)]
    pub app_drawer: AppDrawerIcons,
    #[serde(default)]
    pub keyboard: KeyboardIcons,
    #[serde(default)]
    pub notifications: NotificationIcons,
    #[serde(default)]
    pub power_options: PowerOptionsIcons,
    #[serde(default)]
    pub running_apps: RunningAppsIcons,
    #[serde(default)]
    pub settings_drawer: SettingsDrawerIcons,
    #[serde(default)]
    pub status_bar: StatusBarIcons,
    #[serde(default)]
    pub universal_search: UniversalSearchIcons,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct AppDrawerIcons {
    pub category: PathBuf,
    pub default_app: PathBuf,
    pub delete: PathBuf,
    pub info: PathBuf,
    pub search: PathBuf,
    pub x: PathBuf,
}

impl Default for AppDrawerIcons {
    fn default() -> Self {
        Self {
            category: "assets/icons/app-drawer/category.png".into(),
            default_app: "assets/icons/app-drawer/default-app.png".into(),
            delete: "assets/icons/app-drawer/delete.svg".into(),
            info: "assets/icons/app-drawer/info.png".into(),
            search: "assets/icons/app-drawer/search.svg".into(),
            x: "assets/icons/app-drawer/x.png".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct KeyboardIcons {
    pub backspace: PathBuf,
    pub shift: PathBuf,
}

impl Default for KeyboardIcons {
    fn default() -> Self {
        Self {
            backspace: "assets/icons/keyboard/backspace.svg".into(),
            shift: "assets/icons/keyboard/shift.svg".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NotificationIcons {
    pub application: PathBuf,
    pub info: PathBuf,
    pub close: PathBuf,
    pub navbar_gray: PathBuf,
    pub navbar: PathBuf,
}

impl Default for NotificationIcons {
    fn default() -> Self {
        Self {
            application: "assets/icons/notifications/application.svg".into(),
            info: "assets/icons/notifications/info.svg".into(),
            close: "assets/icons/notifications/close.svg".into(),
            navbar_gray: "assets/icons/notifications/navbar-gray.png".into(),
            navbar: "assets/icons/notifications/navbar.png".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct PowerOptionsIcons {
    pub down_arrow: PathBuf,
    pub power_off: PathBuf,
}

impl Default for PowerOptionsIcons {
    fn default() -> Self {
        Self {
            down_arrow: "assets/icons/power-options/down-arrow.svg".into(),
            power_off: "assets/icons/power-options/power-off.svg".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct RunningAppsIcons {
    pub cleanup: PathBuf,
}

impl Default for RunningAppsIcons {
    fn default() -> Self {
        Self {
            cleanup: "assets/icons/running-apps/cleanup.svg".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SettingsDrawerIcons {
    pub airplane: PathBuf,
    pub auto_brightness: PathBuf,
    pub bluetooth_connected: PathBuf,
    pub bluetooth_off: PathBuf,
    pub bluetooth_on: PathBuf,
    pub brightness_high: PathBuf,
    pub brightness_low: PathBuf,
    pub brightness_medium: PathBuf,
    pub camera_off: PathBuf,
    pub camera_on: PathBuf,
    pub cell_signal_high: PathBuf,
    pub cell_signal_none: PathBuf,
    pub cell_signal_warning: PathBuf,
    pub connected_wireless_high_locked: PathBuf,
    pub connected_wireless_high: PathBuf,
    pub connected_wireless_low_locked: PathBuf,
    pub connected_wireless_low: PathBuf,
    pub connected_wireless_medium_locked: PathBuf,
    pub connected_wireless_medium: PathBuf,
    pub connected_wireless_on: PathBuf,
    pub connected_wireless_warning: PathBuf,
    pub connected: PathBuf,
    pub dark_mode: PathBuf,
    pub extended_detected: PathBuf,
    pub extended_only: PathBuf,
    pub external_speaker: PathBuf,
    pub gray_dot_grid: PathBuf,
    pub headphone: PathBuf,
    pub high_performance: PathBuf,
    pub low_performance: PathBuf,
    pub microphone_off: PathBuf,
    pub microphone_on: PathBuf,
    pub mirror_screen: PathBuf,
    pub navbar_gray: PathBuf,
    pub navbar: PathBuf,
    pub power_mode_balanced: PathBuf,
    pub power_mode_high: PathBuf,
    pub power_mode_low: PathBuf,
    pub power_off: PathBuf,
    pub rotation_off: PathBuf,
    pub rotation_on: PathBuf,
    pub screen_mirroring_off: PathBuf,
    pub screen_mirroring_on: PathBuf,
    pub screen_recording_off: PathBuf,
    pub screen_recording_on: PathBuf,
    pub second_screen: PathBuf,
    pub settings: PathBuf,
    pub slider_gray_dot_column: PathBuf,
    pub slider_orange_dot_column: PathBuf,
    pub system_speaker: PathBuf,
    pub terminal: PathBuf,
    pub volume_high: PathBuf,
    pub volume_low: PathBuf,
    pub volume_medium: PathBuf,
    pub volume_off: PathBuf,
    pub wireless_high_locked: PathBuf,
    pub wireless_high: PathBuf,
    pub wireless_low_locked: PathBuf,
    pub wireless_low: PathBuf,
    pub wireless_medium_locked: PathBuf,
    pub wireless_medium: PathBuf,
    pub wireless_none: PathBuf,
    pub wireless_off: PathBuf,
    pub wireless_warning: PathBuf,
}

impl Default for SettingsDrawerIcons {
    fn default() -> Self {
        Self {
            airplane: "assets/icons/settings-drawer/airplane.svg".into(),
            auto_brightness: "assets/icons/settings-drawer/auto-brightness.svg".into(),
            bluetooth_connected: "assets/icons/settings-drawer/bluetooth-connected.svg".into(),
            bluetooth_off: "assets/icons/settings-drawer/bluetooth-off.svg".into(),
            bluetooth_on: "assets/icons/settings-drawer/bluetooth-on.svg".into(),
            brightness_high: "assets/icons/settings-drawer/brightness-high.svg".into(),
            brightness_low: "assets/icons/settings-drawer/brightness-low.svg".into(),
            brightness_medium: "assets/icons/settings-drawer/brightness-medium.svg".into(),
            camera_off: "assets/icons/settings-drawer/camera-off.svg".into(),
            camera_on: "assets/icons/settings-drawer/camera-on.svg".into(),
            cell_signal_high: "assets/icons/settings-drawer/cell-signal-high.svg".into(),
            cell_signal_none: "assets/icons/settings-drawer/cell-signal-none.svg".into(),
            cell_signal_warning: "assets/icons/settings-drawer/cell-signal-warning.svg".into(),
            connected_wireless_high_locked:
                "assets/icons/settings-drawer/connected-wireless-high-locked.svg".into(),
            connected_wireless_high: "assets/icons/settings-drawer/connected-wireless-high.svg"
                .into(),
            connected_wireless_low_locked:
                "assets/icons/settings-drawer/connected-wireless-low-locked.svg".into(),
            connected_wireless_low: "assets/icons/settings-drawer/connected-wireless-low.svg"
                .into(),
            connected_wireless_medium_locked:
                "assets/icons/settings-drawer/connected-wireless-medium-locked.svg".into(),
            connected_wireless_medium: "assets/icons/settings-drawer/connected-wireless-medium.svg"
                .into(),
            connected_wireless_on: "assets/icons/settings-drawer/connected-wireless-on.svg".into(),
            connected_wireless_warning:
                "assets/icons/settings-drawer/connected-wireless-warning.svg".into(),
            connected: "assets/icons/settings-drawer/connected.svg".into(),
            dark_mode: "assets/icons/settings-drawer/dark-mode.svg".into(),
            extended_detected: "assets/icons/settings-drawer/extended-detected.svg".into(),
            extended_only: "assets/icons/settings-drawer/extended-only.svg".into(),
            external_speaker: "assets/icons/settings-drawer/external-speaker.svg".into(),
            gray_dot_grid: "assets/icons/settings-drawer/gray-dot-grid.png".into(),
            headphone: "assets/icons/settings-drawer/headphone.svg".into(),
            high_performance: "assets/icons/settings-drawer/high-performance.svg".into(),
            low_performance: "assets/icons/settings-drawer/low-performance.svg".into(),
            microphone_off: "assets/icons/settings-drawer/microphone-off.svg".into(),
            microphone_on: "assets/icons/settings-drawer/microphone-on.svg".into(),
            mirror_screen: "assets/icons/settings-drawer/mirror-screen.svg".into(),
            navbar_gray: "assets/icons/settings-drawer/navbar-gray.png".into(),
            navbar: "assets/icons/settings-drawer/navbar.png".into(),
            power_mode_balanced: "assets/icons/settings-drawer/power-mode-balanced.svg".into(),
            power_mode_high: "assets/icons/settings-drawer/power-mode-high.svg".into(),
            power_mode_low: "assets/icons/settings-drawer/power-mode-low.svg".into(),
            power_off: "assets/icons/settings-drawer/power-off.svg".into(),
            rotation_off: "assets/icons/settings-drawer/rotation-off.svg".into(),
            rotation_on: "assets/icons/settings-drawer/rotation-on.svg".into(),
            screen_mirroring_off: "assets/icons/settings-drawer/screen-mirroring-off.svg".into(),
            screen_mirroring_on: "assets/icons/settings-drawer/screen-mirroring-on.svg".into(),
            screen_recording_off: "assets/icons/settings-drawer/screen-recording-off.svg".into(),
            screen_recording_on: "assets/icons/settings-drawer/screen-recording-on.svg".into(),
            second_screen: "assets/icons/settings-drawer/second-screen.svg".into(),
            settings: "assets/icons/settings-drawer/settings.svg".into(),
            slider_gray_dot_column: "assets/icons/settings-drawer/slider-gray-dot-column.png"
                .into(),
            slider_orange_dot_column: "assets/icons/settings-drawer/slider-orange-dot-column.png"
                .into(),
            system_speaker: "assets/icons/settings-drawer/system-speaker.svg".into(),
            terminal: "assets/icons/settings-drawer/terminal.svg".into(),
            volume_high: "assets/icons/settings-drawer/volume-high.svg".into(),
            volume_low: "assets/icons/settings-drawer/volume-low.svg".into(),
            volume_medium: "assets/icons/settings-drawer/volume-medium.svg".into(),
            volume_off: "assets/icons/settings-drawer/volume-off.svg".into(),
            wireless_high_locked: "assets/icons/settings-drawer/wireless-high-locked.svg".into(),
            wireless_high: "assets/icons/settings-drawer/wireless-high.svg".into(),
            wireless_low_locked: "assets/icons/settings-drawer/wireless-low-locked.svg".into(),
            wireless_low: "assets/icons/settings-drawer/wireless-low.svg".into(),
            wireless_medium_locked: "assets/icons/settings-drawer/wireless-medium-locked.svg"
                .into(),
            wireless_medium: "assets/icons/settings-drawer/wireless-medium.svg".into(),
            wireless_none: "assets/icons/settings-drawer/wireless-none.svg".into(),
            wireless_off: "assets/icons/settings-drawer/wireless-off.svg".into(),
            wireless_warning: "assets/icons/settings-drawer/wireless-warning.svg".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct StatusBarIcons {
    pub battery_0_charging: PathBuf,
    pub battery_10_charging: PathBuf,
    pub battery_10: PathBuf,
    pub battery_100_charging: PathBuf,
    pub battery_100: PathBuf,
    pub battery_20_charging: PathBuf,
    pub battery_20: PathBuf,
    pub battery_30_charging: PathBuf,
    pub battery_30: PathBuf,
    pub battery_40_charging: PathBuf,
    pub battery_40: PathBuf,
    pub battery_50_charging: PathBuf,
    pub battery_50: PathBuf,
    pub battery_60_charging: PathBuf,
    pub battery_60: PathBuf,
    pub battery_70_charging: PathBuf,
    pub battery_70: PathBuf,
    pub battery_80_charging: PathBuf,
    pub battery_80: PathBuf,
    pub battery_90_charging: PathBuf,
    pub battery_90: PathBuf,
    pub battery_empty: PathBuf,
    pub bluetooth_connected: PathBuf,
    pub bluetooth_off: PathBuf,
    pub bluetooth_on: PathBuf,
    pub bluetooth_warning: PathBuf,
    pub wireless_high: PathBuf,
    pub wireless_low: PathBuf,
    pub wireless_medium: PathBuf,
    pub wireless_off: PathBuf,
    pub wireless_on: PathBuf,
    pub wireless_warning: PathBuf,
}

impl Default for StatusBarIcons {
    fn default() -> Self {
        Self {
            battery_0_charging: "assets/icons/status-bar/battery-0-charging.svg".into(),
            battery_10_charging: "assets/icons/status-bar/battery-10-charging.svg".into(),
            battery_10: "assets/icons/status-bar/battery-10.svg".into(),
            battery_100_charging: "assets/icons/status-bar/battery-100-charging.svg".into(),
            battery_100: "assets/icons/status-bar/battery-100.svg".into(),
            battery_20_charging: "assets/icons/status-bar/battery-20-charging.svg".into(),
            battery_20: "assets/icons/status-bar/battery-20.svg".into(),
            battery_30_charging: "assets/icons/status-bar/battery-30-charging.svg".into(),
            battery_30: "assets/icons/status-bar/battery-30.svg".into(),
            battery_40_charging: "assets/icons/status-bar/battery-40-charging.svg".into(),
            battery_40: "assets/icons/status-bar/battery-40.svg".into(),
            battery_50_charging: "assets/icons/status-bar/battery-50-charging.svg".into(),
            battery_50: "assets/icons/status-bar/battery-50.svg".into(),
            battery_60_charging: "assets/icons/status-bar/battery-60-charging.svg".into(),
            battery_60: "assets/icons/status-bar/battery-60.svg".into(),
            battery_70_charging: "assets/icons/status-bar/battery-70-charging.svg".into(),
            battery_70: "assets/icons/status-bar/battery-70.svg".into(),
            battery_80_charging: "assets/icons/status-bar/battery-80-charging.svg".into(),
            battery_80: "assets/icons/status-bar/battery-80.svg".into(),
            battery_90_charging: "assets/icons/status-bar/battery-90-charging.svg".into(),
            battery_90: "assets/icons/status-bar/battery-90.svg".into(),
            battery_empty: "assets/icons/status-bar/battery-empty.svg".into(),
            bluetooth_connected: "assets/icons/status-bar/bluetooth-connected.svg".into(),
            bluetooth_off: "assets/icons/status-bar/bluetooth-off.svg".into(),
            bluetooth_on: "assets/icons/status-bar/bluetooth-on.svg".into(),
            bluetooth_warning: "assets/icons/status-bar/bluetooth-warning.svg".into(),
            wireless_high: "assets/icons/status-bar/wireless-high.svg".into(),
            wireless_low: "assets/icons/status-bar/wireless-low.svg".into(),
            wireless_medium: "assets/icons/status-bar/wireless-medium.svg".into(),
            wireless_off: "assets/icons/status-bar/wireless-off.svg".into(),
            wireless_on: "assets/icons/status-bar/wireless-on.svg".into(),
            wireless_warning: "assets/icons/status-bar/wireless-warning.svg".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct UniversalSearchIcons {
    pub ardour: PathBuf,
    pub arrow_counter_clock_wise: PathBuf,
    pub arrow_up_right: PathBuf,
    pub audio_file: PathBuf,
    pub chromium: PathBuf,
    pub code_file: PathBuf,
    pub csv_file: PathBuf,
    pub default_app: PathBuf,
    pub default_file: PathBuf,
    pub default_folder: PathBuf,
    pub doc_file: PathBuf,
    pub firefox: PathBuf,
    pub github: PathBuf,
    pub image_file: PathBuf,
    pub locked_file: PathBuf,
    pub navbar: PathBuf,
    pub pdf_file: PathBuf,
    pub search: PathBuf,
    pub video_file: PathBuf,
    pub x: PathBuf,
    pub xls_file: PathBuf,
    pub zip_file: PathBuf,
}

impl Default for UniversalSearchIcons {
    fn default() -> Self {
        Self {
            ardour: "assets/icons/universal-search/ardour.png".into(),
            arrow_counter_clock_wise: "assets/icons/universal-search/arrow-counter-clock-wise.svg"
                .into(),
            arrow_up_right: "assets/icons/universal-search/arrow-up-right.svg".into(),
            audio_file: "assets/icons/universal-search/audio-file.svg".into(),
            chromium: "assets/icons/universal-search/chromium.png".into(),
            code_file: "assets/icons/universal-search/code-file.svg".into(),
            csv_file: "assets/icons/universal-search/csv-file.svg".into(),
            default_app: "assets/icons/universal-search/default-app.svg".into(),
            default_file: "assets/icons/universal-search/default-file.svg".into(),
            default_folder: "assets/icons/universal-search/default-folder.svg".into(),
            doc_file: "assets/icons/universal-search/doc-file.svg".into(),
            firefox: "assets/icons/universal-search/firefox.png".into(),
            github: "assets/icons/universal-search/github.png".into(),
            image_file: "assets/icons/universal-search/image-file.svg".into(),
            locked_file: "assets/icons/universal-search/locked-file.svg".into(),
            navbar: "assets/icons/universal-search/navbar.png".into(),
            pdf_file: "assets/icons/universal-search/pdf-file.svg".into(),
            search: "assets/icons/universal-search/search.svg".into(),
            video_file: "assets/icons/universal-search/video-file.svg".into(),
            x: "assets/icons/universal-search/x.svg".into(),
            xls_file: "assets/icons/universal-search/xls-file.svg".into(),
            zip_file: "assets/icons/universal-search/zip-file.svg".into(),
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
        "/usr/share/mechanix/shell/assets/{}",
        file_name
    )));
    config_paths.push(PathBuf::from(format!(
        "/etc/mechanix/shell/assets/{}",
        file_name
    )));

    if let Some(home_dir) = dirs::home_dir() {
        config_paths.push(home_dir.join(format!(".config/mechanix/shell/assets/{}", file_name)));
    }

    config_paths
}

pub fn load_icons<T>(config_paths: Vec<PathBuf>) -> T
where
    T: for<'de> serde::de::Deserialize<'de> + Default,
{
    let mut merged: Value = Table::new().into();

    for raw_path in config_paths {
        let content = match std::fs::read_to_string(&raw_path) {
            Ok(c) => c,
            Err(_e) => {
                // println!("Could not read config file {}: {}", raw_path.display(), e);
                continue;
            }
        };

        if let Err(_e) = toml::from_str::<T>(&content) {
            // println!("Deserialization failed for {}: {}", raw_path.display(), e);
            continue;
        }

        let table: Table = match content.parse() {
            Ok(t) => t,
            Err(_e) => {
                // println!("Could not parse TOML from {}: {}", raw_path.display(), e);
                continue;
            }
        };

        merged = match merge(merged.clone().into(), table.into()) {
            Ok(v) => v,
            Err(_e) => {
                // println!("Merge failed for {}: {:?}", raw_path.display(), e);
                continue;
            }
        };
    }

    if !merged.as_array().iter().len() == 0 {
        merged.clone().try_into().unwrap_or_default()
    } else {
        T::default()
    }
}
