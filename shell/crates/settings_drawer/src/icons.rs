use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct SettingsDrawerIcons {
    // Icons & UI Images
    #[asset(key = "image.right_nav_bar")]
    pub right_nav_bar: Handle<Image>,

    #[asset(key = "image.airplane_on")]
    pub airplane_on: Handle<Image>,

    #[asset(key = "image.airplane_off")]
    pub airplane_off: Handle<Image>,

    #[asset(key = "image.rotation_on")]
    pub rotation_on: Handle<Image>,

    #[asset(key = "image.rotation_off")]
    pub rotation_off: Handle<Image>,

    #[asset(key = "image.second_screen_on")]
    pub second_screen_on: Handle<Image>,

    #[asset(key = "image.second_screen_off")]
    pub second_screen_off: Handle<Image>,

    #[asset(key = "image.power_mode_low")]
    pub power_mode_low: Handle<Image>,

    #[asset(key = "image.power_mode_high")]
    pub power_mode_high: Handle<Image>,

    #[asset(key = "image.mic_on")]
    pub mic_on: Handle<Image>,

    #[asset(key = "image.mic_off")]
    pub mic_off: Handle<Image>,

    #[asset(key = "image.screen_recording_on")]
    pub screen_recording_on: Handle<Image>,

    #[asset(key = "image.screen_recording_off")]
    pub screen_recording_off: Handle<Image>,

    #[asset(key = "image.calculator")]
    pub calculator: Handle<Image>,

    #[asset(key = "image.camera")]
    pub camera: Handle<Image>,

    #[asset(key = "image.sound_low")]
    pub sound_low: Handle<Image>,

    #[asset(key = "image.brightness_low")]
    pub brightness_low: Handle<Image>,

    #[asset(key = "image.wireless_off")]
    pub wireless_off: Handle<Image>,

    #[asset(key = "image.wireless_on")]
    pub wireless_on: Handle<Image>,

    #[asset(key = "image.wireless_low")]
    pub wireless_low: Handle<Image>,

    #[asset(key = "image.wireless_medium")]
    pub wireless_medium: Handle<Image>,

    #[asset(key = "image.wireless_high")]
    pub wireless_high: Handle<Image>,

    #[asset(key = "image.wireless_full")]
    pub wireless_full: Handle<Image>,

    #[asset(key = "image.bluetooth_off")]
    pub bluetooth_off: Handle<Image>,

    #[asset(key = "image.bluetooth_on")]
    pub bluetooth_on: Handle<Image>,

    #[asset(key = "image.terminal")]
    pub terminal: Handle<Image>,

    #[asset(key = "image.cell_signal_high")]
    pub cell_signal_high: Handle<Image>,

    #[asset(key = "image.cell_signal_none")]
    pub cell_signal_none: Handle<Image>,

    #[asset(key = "image.battery_10")]
    pub battery_10: Handle<Image>,

    #[asset(key = "image.settings")]
    pub settings: Handle<Image>,

    #[asset(key = "image.power")]
    pub power: Handle<Image>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum SettingsDrawerIconsState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct SettingsDrawerIconsPlugin;

impl bevy::prelude::Plugin for SettingsDrawerIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SettingsDrawerIconsState>();
        app.add_loading_state(
            LoadingState::new(SettingsDrawerIconsState::Loading)
                .continue_to_state(SettingsDrawerIconsState::Loaded)
                .on_failure_continue_to_state(SettingsDrawerIconsState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("icons.ron")
                .load_collection::<SettingsDrawerIcons>(),
        );
    }
}

pub fn icons_loaded(load_state: Res<State<SettingsDrawerIconsState>>) -> bool {
    *load_state == SettingsDrawerIconsState::Loaded
}
