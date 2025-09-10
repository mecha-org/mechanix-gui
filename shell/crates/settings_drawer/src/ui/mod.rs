mod drawer_items;
mod navigation_bar;

use crate::icons::SettingsDrawerIcons;
pub use crate::ui::{drawer_items::*, navigation_bar::*};
use bevy::prelude::*;
use service_plugins::{
    BluetoothPlugin, NetworkManagerPlugin, PulseAudioPlugin, upower::UPowerPlugin,
};
use types::prelude::FontAssets;

pub const BAR_SIZE: (f32, f32) = (180., 30.);
pub const DRAWER_ITEMS_SIZE: (f32, f32) = (540., 584.);

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, States)]
pub enum SettingsDrawerState {
    #[default]
    AssetsLoading,
    NavigationOnly,
    Opened,
}

pub fn init_state(mut commands: Commands, mut done: Local<bool>) {
    if *done {
        return;
    }

    commands.set_state(SettingsDrawerState::NavigationOnly);

    *done = true;
}

#[derive(Component)]
pub struct SettingsDrawerRoot;

pub fn ui(
    commands: &mut Commands,
    font_assets: &FontAssets,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        SettingsDrawerRoot,
    )
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WirelessUiPlugin,
            BluetoothUiPlugin,
            RotationUiPlugin,
            AirplaneModeUiPlugin,
            MicrophoneUiPlugin,
            ScreenRecodingUiPlugin,
            PowerSavingModeUiPlugin,
            ScreenSharingUiPlugin,
            CellularUiPlugin,
            BrightnessUiPlugin,
            VolumeUiPlugin,
        ));
        app.add_systems(Update, update_button_type1_styles);
        app.add_systems(Update, update_button_type2_styles);
        app.add_systems(Update, update_button_type3_styles);
        app.add_systems(Update, update_button_type4_styles);
    }
}

pub struct ServicesPlugins;

impl Plugin for ServicesPlugins {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<NetworkManagerPlugin>() {
            app.add_plugins(NetworkManagerPlugin);
        }
        if !app.is_plugin_added::<BluetoothPlugin>() {
            app.add_plugins(BluetoothPlugin);
        }
        if !app.is_plugin_added::<UPowerPlugin>() {
            app.add_plugins(UPowerPlugin);
        }
        if !app.is_plugin_added::<PulseAudioPlugin>() {
            app.add_plugins(PulseAudioPlugin);
        }
    }
}
