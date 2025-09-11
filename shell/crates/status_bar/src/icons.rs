use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct StatusBarIcons {
    //Wireless Icons
    #[asset(key = "icons.wireless_on")]
    pub wifi_on: Handle<Image>,
    #[asset(key = "icons.wireless_off")]
    pub wifi_off: Handle<Image>,

    #[asset(key = "icons.wireless_high")]
    pub wifi_high: Handle<Image>,

    #[asset(key = "icons.wireless_medium")]
    pub wifi_medium: Handle<Image>,

    #[asset(key = "icons.wireless_low")]
    pub wifi_low: Handle<Image>,

    #[asset(key = "icons.wireless_fix")]
    pub wifi_fix: Handle<Image>,

    //Bluetooth Icons
    #[asset(key = "icons.bluetooth_on")]
    pub bluetooth_on: Handle<Image>,
    #[asset(key = "icons.bluetooth_off")]
    pub bluetooth_off: Handle<Image>,
    #[asset(key = "icons.bluetooth_connected")]
    pub bluetooth_connected: Handle<Image>,

    //Battery Icons
    #[asset(key = "icons.battery_10")]
    pub battery_10: Handle<Image>,
    #[asset(key = "icons.battery_20")]
    pub battery_20: Handle<Image>,
    #[asset(key = "icons.battery_30")]
    pub battery_30: Handle<Image>,
    #[asset(key = "icons.battery_40")]
    pub battery_40: Handle<Image>,
    #[asset(key = "icons.battery_50")]
    pub battery_50: Handle<Image>,
    #[asset(key = "icons.battery_60")]
    pub battery_60: Handle<Image>,
    #[asset(key = "icons.battery_70")]
    pub battery_70: Handle<Image>,
    #[asset(key = "icons.battery_80")]
    pub battery_80: Handle<Image>,
    #[asset(key = "icons.battery_90")]
    pub battery_90: Handle<Image>,
    #[asset(key = "icons.battery_100")]
    pub battery_100: Handle<Image>,
    #[asset(key = "icons.battery_empty")]
    pub battery_empty: Handle<Image>,
    #[asset(key = "icons.battery_0_charging")]
    pub battery_0_charging: Handle<Image>,
    #[asset(key = "icons.battery_10_charging")]
    pub battery_10_charging: Handle<Image>,
    #[asset(key = "icons.battery_20_charging")]
    pub battery_20_charging: Handle<Image>,
    #[asset(key = "icons.battery_30_charging")]
    pub battery_30_charging: Handle<Image>,
    #[asset(key = "icons.battery_40_charging")]
    pub battery_40_charging: Handle<Image>,
    #[asset(key = "icons.battery_50_charging")]
    pub battery_50_charging: Handle<Image>,
    #[asset(key = "icons.battery_60_charging")]
    pub battery_60_charging: Handle<Image>,
    #[asset(key = "icons.battery_70_charging")]
    pub battery_70_charging: Handle<Image>,
    #[asset(key = "icons.battery_80_charging")]
    pub battery_80_charging: Handle<Image>,
    #[asset(key = "icons.battery_90_charging")]
    pub battery_90_charging: Handle<Image>,
    #[asset(key = "icons.battery_100_charging")]
    pub battery_100_charging: Handle<Image>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum StatusBarIconsState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct StatusBarIconsPlugin;

impl bevy::prelude::Plugin for StatusBarIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<StatusBarIconsState>();
        app.add_loading_state(
            LoadingState::new(StatusBarIconsState::Loading)
                .continue_to_state(StatusBarIconsState::Loaded)
                .on_failure_continue_to_state(StatusBarIconsState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("status_bar.ron")
                .load_collection::<StatusBarIcons>(),
        );
    }
}

pub fn icons_loaded(load_state: Res<State<StatusBarIconsState>>) -> bool {
    *load_state == StatusBarIconsState::Loaded
}
