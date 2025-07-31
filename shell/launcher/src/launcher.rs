use crate::utils::Icon;
use bevy::{
    asset::AssetMetaCheck, ecs::system::SystemId, prelude::*, scene::ron::de, winit::WinitPlugin,
};
use bevy_asset_loader::prelude::*;
use bevy_smithay::{
    SmithayPlugin, SmithayWindowType,
    prelude::{layer_shell::LayerShellSettings, subsurface::Anchor},
};
use bevy_styled_widgets::{
    StyledWidgetsPlugin,
    prelude::{
        ButtonVariant, StyledButton, StyledButtonPlugin, StyledText, StyledTextPlugin, ThemeManager,
    },
};

use crate::{
    // StyledWidgetsPlugin,
    components::{status_bar, AssetsLoadingState, StatusBarPlugin},
    styled_card::StyledCardPlugin,
    components::{
        AssetsLoadingState, ClockPlugin, NavigationBarPlugin, app_list, apps_grid, navigation_bar,
        status_bar, update_apps_categories, update_apps_list,
    },
    desktop_apps::{DesktopApps, DesktopAppsPlugin},
    styled_card::{StyledCard, StyledCardPlugin},
    utils::FontAssets,
};
use bevy::app::TaskPoolThreadAssignmentPolicy;
use bevy::{prelude::*, winit::WinitPlugin};
use bevy_asset_loader::prelude::*;
use bevy_plugins::bluetooth::BluetoothEnabledStatus;
use bevy_plugins::network_manager::WirelessEnabled;
use bevy_plugins::upower::UPowerPlugin;
use bevy_plugins::{BluetoothPlugin, NetworkManagerPlugin};
use bevy_smithay::{
    prelude::{layer_shell::LayerShellSettings, subsurface::Anchor}, SmithayPlugin,
    SmithayWindowType,
};
use bevy_styled_widgets::prelude::{StyledTextPlugin, ThemeManager};


#[derive(Debug, Component)]
pub struct HomescreenWindow;

#[derive(Debug, Component)]
pub struct StatusBarWindow;

#[derive(Debug, Component)]
pub struct SearchWindow;

#[derive(Debug, Component)]
pub struct NavigationBarWindow;

#[derive(Debug, Component)]
pub struct SettingsDrawerWindow;

#[derive(Debug, Component)]
pub struct AppSwitcherWindow;

#[derive(Debug, Event, PartialEq, Eq)]
pub enum NavigationEvents {
    OpenSettingDrawer,
    AppSwitcher,
    OpenHomescreen,
    OpenSearch,
}

pub fn run_launcher() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        io: TaskPoolThreadAssignmentPolicy {
                            min_threads: 10, //todo: revisit required
                            max_threads: 12,
                            percent: 0.5,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        ..Default::default()
                    }
                })
                .set(WindowPlugin {
                    // Configure the primary window for the status bar
                    primary_window: Some(Window {
                        title: "Status Bar".to_string(),
                        resolution: (540.0, 44.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
                    ..Default::default()
                })
                .disable::<WinitPlugin>(),
            SmithayPlugin {
                primary_window_type: SmithayWindowType::LayerShell {
                    settings: LayerShellSettings {
                        size: (540, 44),
                        layer: bevy_smithay::prelude::subsurface::Layer::Top,
                        exclusive_zone: 44,
                        anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                        ..default()
                    },
                },
            },
        ))
        .insert_resource(ThemeManager::default())
        // Asset loading state and configuration
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup)
        .add_plugins(StatusBarPlugin)
        .add_plugins(StyledCardPlugin)
        .add_plugins(NetworkManagerPlugin)
        .add_plugins(BluetoothPlugin)
        .add_plugins(UPowerPlugin)
        .add_observer(bar_on_click)
        .add_observer(bar_on_drag_start)
        .add_observer(bar_on_drag)
        .add_observer(bar_on_drag_end)
        .add_systems(Update, animate_bar_drag_end)
        // Add core application plugins
        .add_plugins((
            ClockPlugin,
            NavigationBarPlugin,
            StyledCardPlugin,
            StyledWidgetsPlugin,
            LauncherUiPlugin,
            DesktopAppsPlugin,
            // StyledTextPlugin,
            // Custom plugin for organizing UI setup
        ))
        // System to exit on Escape key press
        .add_systems(Update, exit_on_esc)
        .run();
}

fn assets_loaded(load_state: Res<State<AssetsLoadingState>>) -> bool {
    *load_state == AssetsLoadingState::Loaded
}

pub struct LauncherUiPlugin;

impl Plugin for LauncherUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetsLoadingState::Loaded), setup_launcher_ui);
        //insert resource only when the assets are loaded
        app.add_systems(
            OnEnter(AssetsLoadingState::Loaded),
            |mut commands: Commands, asset_server: Res<AssetServer>| {
                let apps = DesktopApps::new(&mut commands, &asset_server);
                commands.insert_resource(apps);
            },
        );

        app.add_systems(
            Update,
            update_apps_list
                .run_if(assets_loaded)
                .run_if(resource_exists_and_changed::<DesktopApps>),
        );
        app.add_systems(
            Update,
            update_apps_categories
                .run_if(assets_loaded)
                .run_if(resource_exists_and_changed::<DesktopApps>),
        );

        app.add_observer(handle_navigation_events);
    }
}

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn setup(mut commands: Commands, theme_manager: Res<ThemeManager>, font_assets: Res<FontAssets>, wireless_enabled: Res<WirelessEnabled>, bluetooth_enabled: Res<BluetoothEnabledStatus>) {
    //Spawn status bar
    let wireless_default_icon = if wireless_enabled.0 {
        Icon::WirelessNone
    } else {
        Icon::WirelessOff
    }.into();
    let bluetooth_default_icon: Icon = if bluetooth_enabled.0 {
        Icon::BluetoothNone
    } else {
        Icon::BluetoothOff
    }.into();
    //Spawn status bar
    //Spawn status bar camera
fn setup_launcher_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    spawn_status_bar_ui(&mut commands, &font_assets);

    spawn_homescreen_window(&mut commands);

    // spawn_settings_drawer(&mut commands);

    spawn_navigation_bar_window(&mut commands);
}

fn spawn_status_bar_ui(commands: &mut Commands, font_assets: &Res<FontAssets>) {
    commands.spawn(Camera2d);
    //Spawn status bar node
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        children![status_bar(&font_assets, wireless_default_icon, bluetooth_default_icon),],
    ));
}

fn spawn_homescreen_window(commands: &mut Commands) {
    let on_click = commands.register_system(
        |mut commands: Commands, q_homescreen: Option<Single<Entity, With<HomescreenWindow>>>| {
            info!("close homescreen clicked");
            if let Some(entity) = q_homescreen {
                commands.entity(entity.into_inner()).despawn();
            }
        },
    );
    let camera = spawn_camera(
        commands,
        540,
        531,
        "Home screen".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Bottom,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        HomescreenWindow,
    );

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        children![apps_grid(),],
    ));
}

fn spawn_navigation_bar_window(commands: &mut Commands) {
    let camera = spawn_camera(
        commands,
        540,
        44,
        "Navigation bar".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Top,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::BOTTOM,
            exclusive_zone: 44,
            ..default()
        },
        NavigationBarWindow,
    );

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        children![navigation_bar()],
    ));
}

fn handle_navigation_events(
    mut trigger: Trigger<NavigationEvents>,
    mut commands: Commands,
    q_homescreen: Option<Single<Entity, With<HomescreenWindow>>>,
) {
    info!("event is {:?}", trigger.event());
    match trigger.event() {
        NavigationEvents::OpenSettingDrawer => {
            spawn_settings_drawer(&mut commands);
        }
        NavigationEvents::AppSwitcher => {
            spawn_app_switcher(&mut commands);
        }
        NavigationEvents::OpenHomescreen => {
            spawn_homescreen_window(&mut commands);
        }
        NavigationEvents::OpenSearch => {
            spawn_search(&mut commands);
        }
    }
}

fn spawn_settings_drawer(commands: &mut Commands) {
    let on_click = commands.register_system(
        |mut commands: Commands,
         q_settings_drawer: Option<Single<Entity, With<SettingsDrawerWindow>>>| {
            info!("close settings panel clicked");
            if let Some(entity) = q_settings_drawer {
                commands.entity(entity.into_inner()).despawn();
            }
        },
    );
    let camera = spawn_camera(
        commands,
        540,
        531,
        "Settings Drawer".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Top,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        SettingsDrawerWindow,
    );

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        spawn_content("Settings Drawer", on_click),
    ));
}

fn spawn_search(commands: &mut Commands) {
    let on_click = commands.register_system(
        |mut commands: Commands, q_search: Option<Single<Entity, With<SearchWindow>>>| {
            info!("close search clicked");
            if let Some(entity) = q_search {
                commands.entity(entity.into_inner()).despawn();
            }
        },
    );
    let camera = spawn_camera(
        commands,
        540,
        531,
        "Search".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Top,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        SearchWindow,
    );

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        spawn_content("Search", on_click),
    ));
}

fn spawn_camera(
    commands: &mut Commands,
    width: u32,
    height: u32,
    title: String,
    mut settings: LayerShellSettings,
    marker: impl Bundle,
) -> Entity {
    settings.size = (width, height);
    let entity = commands
        .spawn((
            Window {
                title: title.into(),
                resolution: (width as f32, height as f32).into(),
                ..default()
            },
            SmithayWindowType::LayerShell { settings },
            marker,
        ))
        .id();

    commands
        .spawn((
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(entity),
                ),
                clear_color: ClearColorConfig::Custom(Color::default()),
                ..default()
            },
            Camera2d,
        ))
        .id()
}

fn spawn_content<S: Into<String>>(title: S, on_click: SystemId) -> impl Bundle {
    children![
        StyledText::builder().content(title).font_size(28.).build(),
        Node {
            margin: UiRect::vertical(Val::Px(10.)),
            ..Default::default()
        },
        StyledButton::builder()
            .text("Close")
            .variant(ButtonVariant::Secondary)
            .on_click(on_click)
            .build()
    ]
}

fn spawn_app_switcher(commands: &mut Commands) {
    let on_click = commands.register_system(
        |mut commands: Commands,
         q_app_switcher: Option<Single<Entity, With<AppSwitcherWindow>>>| {
            info!("close app switcher clicked");
            if let Some(entity) = q_app_switcher {
                commands.entity(entity.into_inner()).despawn();
            }
        },
    );
    let camera = spawn_camera(
        commands,
        540,
        531,
        "App Switcher".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Top,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        AppSwitcherWindow,
    );

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        spawn_content("App Switcher", on_click),
    ));
}
