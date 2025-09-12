mod events;
mod icons;
mod setup;
mod ui;

use animation::{DefaultTweenPlugins, TweenCorePlugin, prelude::TweenEvent};
use bevy::prelude::*;

use bevy_wayland::prelude::InputRegion;
use utils::prelude::{FontAssetsPlugin, fonts_loaded};

use crate::{
    events::{listen_close_event, listen_open_event},
    icons::{SettingsDrawerIconsPlugin, icons_loaded},
    setup::{
        SettingsDrawerWindow, SettingsDrawerWindowCamera, WINDOW_SIZE, camera_setup, exit_on_esc,
    },
    ui::{
        BAR_SIZE, ServicesPlugins, SettingsDrawerState, UiPlugin, despawn_drawer_items, init_state,
        on_bar_drag, spawn_drawer_items, spawn_navigation_bar, update_active_network_strength,
        update_airplane_mode_state, update_bluetooth_state, update_brightness_state,
        update_cellular_state, update_microphone_state, update_power_saving_mode_state,
        update_rotation_state, update_screen_recording_state, update_screen_sharing_state,
        update_volume_state, update_wireless_state,
    },
};

pub struct SettingsDrawerPlugin;
impl Plugin for SettingsDrawerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FontAssetsPlugin>() {
            app.add_plugins(FontAssetsPlugin);
        }
        app.add_plugins(SettingsDrawerIconsPlugin);
        if !app.is_plugin_added::<TweenCorePlugin>() {
            app.add_plugins(DefaultTweenPlugins);
        }
        if !app.is_plugin_added::<headless_widgets::CoreWidgetsPlugin>() {
            app.add_plugins(headless_widgets::CoreWidgetsPlugin);
        }
        app.add_plugins(UiPlugin);
        app.add_plugins(ServicesPlugins);

        app.insert_state(SettingsDrawerState::default());
        app.add_systems(Startup, camera_setup);
        app.add_systems(
            Update,
            setup::setup
                .run_if(resource_exists::<SettingsDrawerWindowCamera>)
                .run_if(fonts_loaded)
                .run_if(icons_loaded),
        );
        app.add_systems(Update, init_state.run_if(fonts_loaded).run_if(icons_loaded));

        app.add_observer(listen_open_event);
        app.add_observer(listen_close_event);
        app.add_systems(
            OnEnter(SettingsDrawerState::NavigationOnly),
            spawn_navigation_bar,
        );
        app.add_systems(
            OnEnter(SettingsDrawerState::Opened),
            (
                spawn_drawer_items,
                update_wireless_state,
                update_active_network_strength,
                update_bluetooth_state,
                update_brightness_state,
                update_cellular_state,
                update_microphone_state,
                update_power_saving_mode_state,
                update_rotation_state,
                update_screen_recording_state,
                update_screen_sharing_state,
                update_volume_state,
                update_airplane_mode_state,
            )
                .chain(),
        );
        app.add_systems(OnExit(SettingsDrawerState::Opened), despawn_drawer_items);

        app.add_systems(Update, listen_animation_events);
        app.add_systems(Update, exit_on_esc);

        app.add_observer(on_bar_drag);
    }
}

fn listen_animation_events(
    mut event_reader: EventReader<TweenEvent<&'static str>>,
    mut q_input_region: Single<&mut InputRegion, With<SettingsDrawerWindow>>,
) {
    event_reader.read().for_each(|event| match event.data {
        "SettingsDrawerOpened" => {
            q_input_region.0 = Rect::new(0., 0., WINDOW_SIZE.0, WINDOW_SIZE.1);
        }
        "SettingsDrawerClosed" => {
            q_input_region.0 = Rect::new(
                WINDOW_SIZE.0 - BAR_SIZE.0,
                WINDOW_SIZE.1 - BAR_SIZE.1,
                WINDOW_SIZE.0,
                WINDOW_SIZE.1,
            );
        }

        _ => (),
    });
}

pub mod prelude {
    pub use crate::SettingsDrawerPlugin;
}
