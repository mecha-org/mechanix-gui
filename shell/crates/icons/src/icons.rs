use std::path::PathBuf;

use commons::prelude::*;
use serde::Deserialize;
use toml::{Table, Value};

const ICONS_BASE_PATH: &str = "/usr/share/mechanix/shell/assets/icons/";

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
    #[serde(default)]
    pub lockscreen: LockscreenIcons,
    #[serde(default)]
    pub homescreen: HomeScreenIcons,
    #[serde(default)]
    pub toast: ToastIcons,
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
            category: PathBuf::from(format!("{}app-drawer/category.png", ICONS_BASE_PATH)),
            default_app: PathBuf::from(format!("{}app-drawer/default-app.png", ICONS_BASE_PATH)),
            delete: PathBuf::from(format!("{}app-drawer/delete.svg", ICONS_BASE_PATH)),
            info: PathBuf::from(format!("{}app-drawer/info.png", ICONS_BASE_PATH)),
            search: PathBuf::from(format!("{}app-drawer/search.svg", ICONS_BASE_PATH)),
            x: PathBuf::from(format!("{}app-drawer/x.png", ICONS_BASE_PATH)),
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
            backspace: PathBuf::from(format!("{}keyboard/backspace.svg", ICONS_BASE_PATH)),
            shift: PathBuf::from(format!("{}keyboard/shift.svg", ICONS_BASE_PATH)),
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
    pub bell: PathBuf,
}

impl Default for NotificationIcons {
    fn default() -> Self {
        Self {
            application: PathBuf::from(format!("{}notifications/application.svg", ICONS_BASE_PATH)),
            info: PathBuf::from(format!("{}notifications/info.svg", ICONS_BASE_PATH)),
            close: PathBuf::from(format!("{}notifications/close.svg", ICONS_BASE_PATH)),
            navbar_gray: PathBuf::from(format!("{}notifications/navbar-gray.png", ICONS_BASE_PATH)),
            navbar: PathBuf::from(format!("{}notifications/navbar.png", ICONS_BASE_PATH)),
            bell: PathBuf::from(format!("{}notifications/bell.svg", ICONS_BASE_PATH)),
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
            down_arrow: PathBuf::from(format!("{}power-options/down-arrow.svg", ICONS_BASE_PATH)),
            power_off: PathBuf::from(format!("{}power-options/power-off.svg", ICONS_BASE_PATH)),
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
            cleanup: PathBuf::from(format!("{}running-apps/cleanup.svg", ICONS_BASE_PATH)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct LockscreenIcons {
    pub arrow: PathBuf,
    pub bell: PathBuf,
    pub lock: PathBuf,
    pub lock_open_full: PathBuf,
    pub lock_open_half: PathBuf,
    pub wedge_left: PathBuf,
    pub wedge_left_outline: PathBuf,
    pub wedge_right: PathBuf,
    pub wedge_right_outline: PathBuf,
    pub wallpaper: PathBuf,
}

impl Default for LockscreenIcons {
    fn default() -> Self {
        Self {
            arrow: PathBuf::from(format!("{}lockscreen/arrow.svg", ICONS_BASE_PATH)),
            bell: PathBuf::from(format!("{}lockscreen/bell.svg", ICONS_BASE_PATH)),
            lock: PathBuf::from(format!("{}lockscreen/lock.svg", ICONS_BASE_PATH)),
            lock_open_full: PathBuf::from(format!(
                "{}lockscreen/lock-open-full.svg",
                ICONS_BASE_PATH
            )),
            lock_open_half: PathBuf::from(format!(
                "{}lockscreen/lock-open-half.svg",
                ICONS_BASE_PATH
            )),
            wedge_left: PathBuf::from(format!("{}lockscreen/wedge-left.svg", ICONS_BASE_PATH)),
            wedge_left_outline: PathBuf::from(format!(
                "{}lockscreen/wedge_left_outline.svg",
                ICONS_BASE_PATH
            )),
            wedge_right: PathBuf::from(format!("{}lockscreen/wedge-right.svg", ICONS_BASE_PATH)),
            wedge_right_outline: PathBuf::from(format!(
                "{}lockscreen/wedge_right_outline.svg",
                ICONS_BASE_PATH
            )),
            wallpaper: PathBuf::from(format!("{}lockscreen/wallpaper.png", ICONS_BASE_PATH)),
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
    pub slider_accent_dots_column: PathBuf,
}

impl Default for SettingsDrawerIcons {
    fn default() -> Self {
        Self {
            airplane: PathBuf::from(format!("{}settings-drawer/airplane.svg", ICONS_BASE_PATH)),
            auto_brightness: PathBuf::from(format!(
                "{}settings-drawer/auto-brightness.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_connected: PathBuf::from(format!(
                "{}settings-drawer/bluetooth-connected.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_off: PathBuf::from(format!(
                "{}settings-drawer/bluetooth-off.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_on: PathBuf::from(format!(
                "{}settings-drawer/bluetooth-on.svg",
                ICONS_BASE_PATH
            )),
            brightness_high: PathBuf::from(format!(
                "{}settings-drawer/brightness-high.svg",
                ICONS_BASE_PATH
            )),
            brightness_low: PathBuf::from(format!(
                "{}settings-drawer/brightness-low.svg",
                ICONS_BASE_PATH
            )),
            brightness_medium: PathBuf::from(format!(
                "{}settings-drawer/brightness-medium.svg",
                ICONS_BASE_PATH
            )),
            camera_off: PathBuf::from(format!("{}settings-drawer/camera-off.svg", ICONS_BASE_PATH)),
            camera_on: PathBuf::from(format!("{}settings-drawer/camera-on.svg", ICONS_BASE_PATH)),
            cell_signal_high: PathBuf::from(format!(
                "{}settings-drawer/cell-signal-high.svg",
                ICONS_BASE_PATH
            )),
            cell_signal_none: PathBuf::from(format!(
                "{}settings-drawer/cell-signal-none.svg",
                ICONS_BASE_PATH
            )),
            cell_signal_warning: PathBuf::from(format!(
                "{}settings-drawer/cell-signal-warning.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_high_locked: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-high-locked.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_high: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-high.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_low_locked: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-low-locked.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_low: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-low.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_medium_locked: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-medium-locked.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_medium: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-medium.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_on: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-on.svg",
                ICONS_BASE_PATH
            )),
            connected_wireless_warning: PathBuf::from(format!(
                "{}settings-drawer/connected-wireless-warning.svg",
                ICONS_BASE_PATH
            )),
            connected: PathBuf::from(format!("{}settings-drawer/connected.svg", ICONS_BASE_PATH)),
            dark_mode: PathBuf::from(format!("{}settings-drawer/dark-mode.svg", ICONS_BASE_PATH)),
            extended_detected: PathBuf::from(format!(
                "{}settings-drawer/extended-detected.svg",
                ICONS_BASE_PATH
            )),
            extended_only: PathBuf::from(format!(
                "{}settings-drawer/extended-only.svg",
                ICONS_BASE_PATH
            )),
            external_speaker: PathBuf::from(format!(
                "{}settings-drawer/external-speaker.svg",
                ICONS_BASE_PATH
            )),
            gray_dot_grid: PathBuf::from(format!(
                "{}settings-drawer/gray-dot-grid.png",
                ICONS_BASE_PATH
            )),
            headphone: PathBuf::from(format!("{}settings-drawer/headphone.svg", ICONS_BASE_PATH)),
            high_performance: PathBuf::from(format!(
                "{}settings-drawer/high-performance.svg",
                ICONS_BASE_PATH
            )),
            low_performance: PathBuf::from(format!(
                "{}settings-drawer/low-performance.svg",
                ICONS_BASE_PATH
            )),
            microphone_off: PathBuf::from(format!(
                "{}settings-drawer/microphone-off.svg",
                ICONS_BASE_PATH
            )),
            microphone_on: PathBuf::from(format!(
                "{}settings-drawer/microphone-on.svg",
                ICONS_BASE_PATH
            )),
            mirror_screen: PathBuf::from(format!(
                "{}settings-drawer/mirror-screen.svg",
                ICONS_BASE_PATH
            )),
            navbar_gray: PathBuf::from(format!(
                "{}settings-drawer/navbar-gray.png",
                ICONS_BASE_PATH
            )),
            navbar: PathBuf::from(format!("{}settings-drawer/navbar.png", ICONS_BASE_PATH)),
            power_mode_balanced: PathBuf::from(format!(
                "{}settings-drawer/power-mode-balanced.svg",
                ICONS_BASE_PATH
            )),
            power_mode_high: PathBuf::from(format!(
                "{}settings-drawer/power-mode-high.svg",
                ICONS_BASE_PATH
            )),
            power_mode_low: PathBuf::from(format!(
                "{}settings-drawer/power-mode-low.svg",
                ICONS_BASE_PATH
            )),
            power_off: PathBuf::from(format!("{}settings-drawer/power-off.svg", ICONS_BASE_PATH)),
            rotation_off: PathBuf::from(format!(
                "{}settings-drawer/rotation-off.svg",
                ICONS_BASE_PATH
            )),
            rotation_on: PathBuf::from(format!(
                "{}settings-drawer/rotation-on.svg",
                ICONS_BASE_PATH
            )),
            screen_mirroring_off: PathBuf::from(format!(
                "{}settings-drawer/screen-mirroring-off.svg",
                ICONS_BASE_PATH
            )),
            screen_mirroring_on: PathBuf::from(format!(
                "{}settings-drawer/screen-mirroring-on.svg",
                ICONS_BASE_PATH
            )),
            screen_recording_off: PathBuf::from(format!(
                "{}settings-drawer/screen-recording-off.svg",
                ICONS_BASE_PATH
            )),
            screen_recording_on: PathBuf::from(format!(
                "{}settings-drawer/screen-recording-on.svg",
                ICONS_BASE_PATH
            )),
            second_screen: PathBuf::from(format!(
                "{}settings-drawer/second-screen.svg",
                ICONS_BASE_PATH
            )),
            settings: PathBuf::from(format!("{}settings-drawer/settings.svg", ICONS_BASE_PATH)),
            slider_gray_dot_column: PathBuf::from(format!(
                "{}settings-drawer/slider-gray-dot-column.png",
                ICONS_BASE_PATH
            )),
            slider_orange_dot_column: PathBuf::from(format!(
                "{}settings-drawer/slider-orange-dot-column.png",
                ICONS_BASE_PATH
            )),
            system_speaker: PathBuf::from(format!(
                "{}settings-drawer/system-speaker.svg",
                ICONS_BASE_PATH
            )),
            terminal: PathBuf::from(format!("{}settings-drawer/terminal.svg", ICONS_BASE_PATH)),
            volume_high: PathBuf::from(format!(
                "{}settings-drawer/volume-high.svg",
                ICONS_BASE_PATH
            )),
            volume_low: PathBuf::from(format!("{}settings-drawer/volume-low.svg", ICONS_BASE_PATH)),
            volume_medium: PathBuf::from(format!(
                "{}settings-drawer/volume-medium.svg",
                ICONS_BASE_PATH
            )),
            volume_off: PathBuf::from(format!("{}settings-drawer/volume-off.svg", ICONS_BASE_PATH)),
            wireless_high_locked: PathBuf::from(format!(
                "{}settings-drawer/wireless-high-locked.svg",
                ICONS_BASE_PATH
            )),
            wireless_high: PathBuf::from(format!(
                "{}settings-drawer/wireless-high.svg",
                ICONS_BASE_PATH
            )),
            wireless_low_locked: PathBuf::from(format!(
                "{}settings-drawer/wireless-low-locked.svg",
                ICONS_BASE_PATH
            )),
            wireless_low: PathBuf::from(format!(
                "{}settings-drawer/wireless-low.svg",
                ICONS_BASE_PATH
            )),
            wireless_medium_locked: PathBuf::from(format!(
                "{}settings-drawer/wireless-medium-locked.svg",
                ICONS_BASE_PATH
            )),
            wireless_medium: PathBuf::from(format!(
                "{}settings-drawer/wireless-medium.svg",
                ICONS_BASE_PATH
            )),
            wireless_none: PathBuf::from(format!(
                "{}settings-drawer/wireless-none.svg",
                ICONS_BASE_PATH
            )),
            wireless_off: PathBuf::from(format!(
                "{}settings-drawer/wireless-off.svg",
                ICONS_BASE_PATH
            )),
            wireless_warning: PathBuf::from(format!(
                "{}settings-drawer/wireless-warning.svg",
                ICONS_BASE_PATH
            )),
            slider_accent_dots_column: PathBuf::from(format!(
                "{}settings-drawer/slider-accent-dots-column.svg",
                ICONS_BASE_PATH
            )),
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
            battery_0_charging: PathBuf::from(format!(
                "{}status-bar/battery-0-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_10_charging: PathBuf::from(format!(
                "{}status-bar/battery-10-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_10: PathBuf::from(format!("{}status-bar/battery-10.svg", ICONS_BASE_PATH)),
            battery_100_charging: PathBuf::from(format!(
                "{}status-bar/battery-100-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_100: PathBuf::from(format!("{}status-bar/battery-100.svg", ICONS_BASE_PATH)),
            battery_20_charging: PathBuf::from(format!(
                "{}status-bar/battery-20-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_20: PathBuf::from(format!("{}status-bar/battery-20.svg", ICONS_BASE_PATH)),
            battery_30_charging: PathBuf::from(format!(
                "{}status-bar/battery-30-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_30: PathBuf::from(format!("{}status-bar/battery-30.svg", ICONS_BASE_PATH)),
            battery_40_charging: PathBuf::from(format!(
                "{}status-bar/battery-40-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_40: PathBuf::from(format!("{}status-bar/battery-40.svg", ICONS_BASE_PATH)),
            battery_50_charging: PathBuf::from(format!(
                "{}status-bar/battery-50-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_50: PathBuf::from(format!("{}status-bar/battery-50.svg", ICONS_BASE_PATH)),
            battery_60_charging: PathBuf::from(format!(
                "{}status-bar/battery-60-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_60: PathBuf::from(format!("{}status-bar/battery-60.svg", ICONS_BASE_PATH)),
            battery_70_charging: PathBuf::from(format!(
                "{}status-bar/battery-70-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_70: PathBuf::from(format!("{}status-bar/battery-70.svg", ICONS_BASE_PATH)),
            battery_80_charging: PathBuf::from(format!(
                "{}status-bar/battery-80-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_80: PathBuf::from(format!("{}status-bar/battery-80.svg", ICONS_BASE_PATH)),
            battery_90_charging: PathBuf::from(format!(
                "{}status-bar/battery-90-charging.svg",
                ICONS_BASE_PATH
            )),
            battery_90: PathBuf::from(format!("{}status-bar/battery-90.svg", ICONS_BASE_PATH)),
            battery_empty: PathBuf::from(format!(
                "{}status-bar/battery-empty.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_connected: PathBuf::from(format!(
                "{}status-bar/bluetooth-connected.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_off: PathBuf::from(format!(
                "{}status-bar/bluetooth-off.svg",
                ICONS_BASE_PATH
            )),
            bluetooth_on: PathBuf::from(format!("{}status-bar/bluetooth-on.svg", ICONS_BASE_PATH)),
            bluetooth_warning: PathBuf::from(format!(
                "{}status-bar/bluetooth-warning.svg",
                ICONS_BASE_PATH
            )),
            wireless_high: PathBuf::from(format!(
                "{}status-bar/wireless-high.svg",
                ICONS_BASE_PATH
            )),
            wireless_low: PathBuf::from(format!("{}status-bar/wireless-low.svg", ICONS_BASE_PATH)),
            wireless_medium: PathBuf::from(format!(
                "{}status-bar/wireless-medium.svg",
                ICONS_BASE_PATH
            )),
            wireless_off: PathBuf::from(format!("{}status-bar/wireless-off.svg", ICONS_BASE_PATH)),
            wireless_on: PathBuf::from(format!("{}status-bar/wireless-on.svg", ICONS_BASE_PATH)),
            wireless_warning: PathBuf::from(format!(
                "{}status-bar/wireless-warning.svg",
                ICONS_BASE_PATH
            )),
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
            ardour: PathBuf::from(format!("{}universal-search/ardour.png", ICONS_BASE_PATH)),
            arrow_counter_clock_wise: PathBuf::from(format!(
                "{}universal-search/arrow-counter-clock-wise.svg",
                ICONS_BASE_PATH
            )),
            arrow_up_right: PathBuf::from(format!(
                "{}universal-search/arrow-up-right.svg",
                ICONS_BASE_PATH
            )),
            audio_file: PathBuf::from(format!(
                "{}universal-search/audio-file.svg",
                ICONS_BASE_PATH
            )),
            chromium: PathBuf::from(format!("{}universal-search/chromium.png", ICONS_BASE_PATH)),
            code_file: PathBuf::from(format!("{}universal-search/code-file.svg", ICONS_BASE_PATH)),
            csv_file: PathBuf::from(format!("{}universal-search/csv-file.svg", ICONS_BASE_PATH)),
            default_app: PathBuf::from(format!(
                "{}universal-search/default-app.svg",
                ICONS_BASE_PATH
            )),
            default_file: PathBuf::from(format!(
                "{}universal-search/default-file.svg",
                ICONS_BASE_PATH
            )),
            default_folder: PathBuf::from(format!(
                "{}universal-search/default-folder.svg",
                ICONS_BASE_PATH
            )),
            doc_file: PathBuf::from(format!("{}universal-search/doc-file.svg", ICONS_BASE_PATH)),
            firefox: PathBuf::from(format!("{}universal-search/firefox.png", ICONS_BASE_PATH)),
            github: PathBuf::from(format!("{}universal-search/github.png", ICONS_BASE_PATH)),
            image_file: PathBuf::from(format!(
                "{}universal-search/image-file.svg",
                ICONS_BASE_PATH
            )),
            locked_file: PathBuf::from(format!(
                "{}universal-search/locked-file.svg",
                ICONS_BASE_PATH
            )),
            navbar: PathBuf::from(format!("{}universal-search/navbar.png", ICONS_BASE_PATH)),
            pdf_file: PathBuf::from(format!("{}universal-search/pdf-file.svg", ICONS_BASE_PATH)),
            search: PathBuf::from(format!("{}universal-search/search.svg", ICONS_BASE_PATH)),
            video_file: PathBuf::from(format!(
                "{}universal-search/video-file.svg",
                ICONS_BASE_PATH
            )),
            x: PathBuf::from(format!("{}universal-search/x.svg", ICONS_BASE_PATH)),
            xls_file: PathBuf::from(format!("{}universal-search/xls-file.svg", ICONS_BASE_PATH)),
            zip_file: PathBuf::from(format!("{}universal-search/zip-file.svg", ICONS_BASE_PATH)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ExtensionsIcons {
    pub gamepad: PathBuf,
    pub keyboard: PathBuf,
    pub gpio: PathBuf,
    pub unknown: PathBuf,
    pub dot_grid: PathBuf,
    pub detached: PathBuf,
}

impl Default for ExtensionsIcons {
    fn default() -> Self {
        Self {
            gamepad: PathBuf::from(format!(
                "{}homescreen/extensions/gamepad.png",
                ICONS_BASE_PATH
            )),
            keyboard: PathBuf::from(format!(
                "{}homescreen/extensions/keyboard.png",
                ICONS_BASE_PATH
            )),
            gpio: PathBuf::from(format!("{}homescreen/extensions/gpio.png", ICONS_BASE_PATH)),
            unknown: PathBuf::from(format!(
                "{}homescreen/extensions/unknown.png",
                ICONS_BASE_PATH
            )),
            dot_grid: PathBuf::from(format!(
                "{}homescreen/extensions/dot-grid.png",
                ICONS_BASE_PATH
            )),
            detached: PathBuf::from(format!(
                "{}homescreen/extensions/detached.png",
                ICONS_BASE_PATH
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ClockIcons {
    pub dashed: PathBuf,
}

impl Default for ClockIcons {
    fn default() -> Self {
        Self {
            dashed: PathBuf::from(format!("{}homescreen/clock/dashed.png", ICONS_BASE_PATH)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ToastIcons {
    pub extension_attached: PathBuf,
    pub extension_detached: PathBuf,
    pub close: PathBuf,
}

impl Default for ToastIcons {
    fn default() -> Self {
        Self {
            extension_attached: PathBuf::from(format!("{}toast/extension_attached.png", ICONS_BASE_PATH)),
            extension_detached: PathBuf::from(format!("{}toast/extension_detached.png", ICONS_BASE_PATH)),
            close: PathBuf::from(format!("{}toast/cross.png", ICONS_BASE_PATH)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct HomeScreenIcons {
    pub extensions: ExtensionsIcons,
    pub clock: ClockIcons,
}

impl Default for HomeScreenIcons {
    fn default() -> Self {
        Self {
            extensions: ExtensionsIcons::default(),
            clock: ClockIcons::default(),
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

    if merged.as_array().iter().len() >= 0 {
        merged.clone().try_into().unwrap_or_default()
    } else {
        T::default()
    }
}
