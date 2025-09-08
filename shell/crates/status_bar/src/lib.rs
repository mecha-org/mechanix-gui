mod components;
mod constants;
mod systems;

use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::{
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use service_plugins::{
    BluetoothPlugin, NetworkManagerPlugin,
    bluetooth::{BluetoothDeviceConnectedStatus, BluetoothEnabledStatus},
    network_manager::{ActiveNetworkStrength, WirelessEnabled},
    upower::{DevicePercentage, DeviceState, UPowerPlugin},
};
use bevy_wayland::prelude::{
    Anchor, InputRegion, KeyboardInteractivity, Layer, LayerShellSettings,
};
use service_plugins::network_manager::NetworkManagerDeviceStatus;
use systems::*;
use types::{AssetsLoadingState, prelude::IconAssets};

use crate::components::spawn_status_bar_ui;

#[derive(Debug, Component)]
pub struct StatusBarWindow;

#[derive(Resource)]
struct ClockUpdateTimer(Timer);
pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, pre_setup);
        app.add_systems(OnEnter(AssetsLoadingState::Loaded), spawn_status_bar_ui);
        app.add_plugins(NetworkManagerPlugin);
        app.add_plugins(BluetoothPlugin);
        app.add_plugins(UPowerPlugin);
        app.init_state::<AssetsLoadingState>().add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<IconAssets>(),
        );
        app.add_systems(Update, exit_on_esc);
        app.insert_resource(ClockUpdateTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, update_clock);
        app.add_systems(
            OnEnter(AssetsLoadingState::Loaded),
            // Update,
            (
                update_wireless_state,
                update_bluetooth_on_powered,
                update_power_icon,
                update_wireless_network_strength.after(update_wireless_state)
            ).after(spawn_status_bar_ui).run_if(resource_exists::<IconAssets>),
        );
        app.add_systems(
            Update,
            update_wireless_state
                .run_if(resource_changed::<WirelessEnabled>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        );
        app.add_systems(
            Update,
            update_wireless_network_strength
                .run_if(resource_changed::<ActiveNetworkStrength>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        )
        .add_systems(
            Update,
            update_wireless_device_status
                .run_if(resource_changed::<NetworkManagerDeviceStatus>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        );
        app.add_systems(
            Update,
            update_bluetooth_on_powered
                .run_if(resource_changed::<BluetoothEnabledStatus>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        );
        app.add_systems(
            Update,
            update_bluetooth_on_connected
                .run_if(resource_changed::<BluetoothDeviceConnectedStatus>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        )
        .add_systems(
            Update,
            update_power_icon
                .run_if(resource_changed::<DevicePercentage>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        );
        app.add_systems(
            Update,
            update_power_icon
                .run_if(resource_changed::<DeviceState>)
                .run_if(in_state(AssetsLoadingState::Loaded)),
        );
        // app.add_systems(
        //     Update,
        //     update_power_icon.run_if(resource_changed::<BatteryLevel>),
        // )
    }
}

pub mod prelude {
    pub use crate::StatusBarPlugin;
}

#[derive(Resource)]
pub struct StatusBarCamera(pub Entity);

pub fn pre_setup(mut commands: Commands) {
    let width = 540.;
    let height = 44.;

    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::TOP,
                layer: Layer::Top,
                exclusive_zone: height as i32,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            InputRegion(Rect::new(0., 0., width, height)),
        ))
        .id();
    let camera_ent = commands
        .spawn((
            Camera2d,
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(window_ent),
                ),
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
        ))
        .id();
    commands.insert_resource(StatusBarCamera(camera_ent));
}
