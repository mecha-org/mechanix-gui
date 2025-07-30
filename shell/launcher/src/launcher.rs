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
    components::{
        AssetsLoadingState, ClockPlugin, FrequentlyUsedApps, NavigationBarPlugin, RecentSearches,
        SettingsDrawerPlugin, apps_grid, navigation_bar, settings_drawer, status_bar,
        universal_search, update_apps_categories, update_apps_list, wireless_list_popup,
    },
    desktop_apps::{self, DesktopApp, DesktopApps, DesktopAppsPlugin},
    // sprites_button::{SpritesButtonPlugin, sprites_button_demo},
    styled_card::{StyledCard, StyledCardPlugin},
    utils::FontAssets,
    widgets::LauncherStyledWidgetsPlugin,
};

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

/// Starts the Bevy launcher UI application.
///

pub fn run_launcher() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
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
                .set(ImagePlugin::default_nearest())
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
        // Add core application plugins
        .add_plugins((
            ClockPlugin,
            NavigationBarPlugin,
            StyledCardPlugin,
            StyledWidgetsPlugin,
            LauncherUiPlugin,
            DesktopAppsPlugin,
            // SpritesButtonPlugin,
            LauncherStyledWidgetsPlugin,
            SettingsDrawerPlugin, // StyledTextPlugin,
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
        app.insert_resource(FrequentlyUsedApps(vec![
            "firefox_firefox".to_string(),
            "code".to_string(),
            "microsoft-edge".to_string(),
            "discord_discord".to_string(),
            "zulip_zulip".to_string(),
        ]));

        app.insert_resource(RecentSearches(vec![
            "sc".to_string(),
            "code".to_string(),
            "microsoft".to_string(),
            "discord".to_string(),
            "zulip".to_string(),
        ]));

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

fn setup_launcher_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    theme_manager: Res<ThemeManager>,
) {
    spawn_status_bar_ui(&mut commands, &font_assets);

    // spawn_homescreen_window(&mut commands, &asset_server, &font_assets);

    //  spawn_settings_drawer(&mut commands, &theme_manager);

    spawn_navigation_bar_window(&mut commands);
}

fn spawn_status_bar_ui(commands: &mut Commands, font_assets: &Res<FontAssets>) {
    commands.spawn((Camera2d, StatusBarWindow));

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![status_bar(font_assets)],
    ));
}

fn spawn_homescreen_window(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_assets: &FontAssets,
) {
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

    // let sprites_button = sprites_button_demo(commands, asset_server, font_assets);

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
    q_status_bar: Option<Single<Entity, With<StatusBarWindow>>>,
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
    theme_manager: Res<ThemeManager>,
    desktop_apps: Res<DesktopApps>,
    f_used_apps: Res<FrequentlyUsedApps>,
    recent_searches: Res<RecentSearches>,
) {
    info!("event is {:?}", trigger.event());
    match trigger.event() {
        NavigationEvents::OpenSettingDrawer => {
            // if let Some(entity) = q_status_bar {
            //     commands.entity(entity.into_inner()).despawn();
            // }
            spawn_settings_drawer(&mut commands, &theme_manager);
        }
        NavigationEvents::AppSwitcher => {
            spawn_app_switcher(&mut commands);
        }
        NavigationEvents::OpenHomescreen => {
            spawn_homescreen_window(&mut commands, &asset_server, &font_assets);
        }
        NavigationEvents::OpenSearch => {
            let mut filtered_apps = Vec::new();
            for app in f_used_apps.0.iter() {
                desktop_apps
                    .apps
                    .iter()
                    .find(|a| a.app_id == app.to_string())
                    .map(|app| {
                        filtered_apps.push(app.clone());
                    });
            }
            spawn_search(&mut commands, filtered_apps, recent_searches.0.clone());
        }
    }
}

fn spawn_settings_drawer(commands: &mut Commands, theme_manager: &ThemeManager) {
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

    let settings_drawer = settings_drawer(commands, theme_manager);

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
        children![settings_drawer],
    ));
}

fn spawn_search(
    commands: &mut Commands,
    freq_used_apps: Vec<DesktopApp>,
    recent_searches: Vec<String>,
) {
    let camera = spawn_camera(
        commands,
        540,
        531,
        "Search".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Bottom,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        SearchWindow,
    );

    let universal_search = universal_search(freq_used_apps, recent_searches);

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
        children![universal_search],
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
