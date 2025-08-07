use bevy::app::TaskPoolThreadAssignmentPolicy;
use bevy::asset::AssetPath;
use bevy::{
    asset::AssetMetaCheck, ecs::system::SystemId, prelude::*, scene::ron::de, winit::WinitPlugin,
};
use crate::components::{
    search_result_item,  SearchResult, SearchResultsComponent,
};
use bevy_asset_loader::prelude::*;
use bevy_plugins::bluetooth::BluetoothEnabledStatus;
use bevy_plugins::mxsearch::{AppSearchResult, SearchResultType};
use bevy_plugins::network_manager::{NetworkManagerDeviceStatus, WirelessEnabled};
use bevy_plugins::upower::UPowerPlugin;
use bevy_plugins::{
    BluetoothPlugin, MxSearchAction, MxSearchActionEvent, NetworkManagerPlugin,
    MxSearchPlugin,
};
use bevy_smithay::{
    SmithayPlugin, SmithayWindowType,
    prelude::{layer_shell::LayerShellSettings, subsurface::Anchor},
};
use bevy_styled_widgets::{
    prelude::{
        ButtonVariant, StyledButton, StyledButtonPlugin, StyledText, StyledTextPlugin, ThemeManager,
    },
    StyledWidgetsPlugin,
};
use freedesktop_icons::lookup;
use std::path::Path;
use bevy_plugins::mxsearch::FileSearchResult;
use crate::components::{FrequentlyUsedApps, RecentSearches, SettingsDrawerPlugin, settings_drawer, universal_search, AppSearchResultUiResource, SearchText};
use crate::utils::Icon;
use crate::{
    components::{
        AssetsLoadingState, NavigationBarPlugin, StatusBarPlugin, app_list, apps_grid,
        navigation_bar, status_bar, update_apps_categories, update_apps_list,
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
                    },
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
            StatusBarPlugin,
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
        .add_plugins(NetworkManagerPlugin)
        .add_plugins(BluetoothPlugin)
        .add_plugins(UPowerPlugin)
        .add_plugins(MxSearchPlugin)
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
            "terminal".to_string(),
            "discord".to_string(),
            "zulip".to_string(),
            // "terminal".to_string(),
        ]));
        app.insert_resource(AppSearchResultUiResource::default());
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
            search_text_updated.run_if(resource_exists_and_changed::<SearchText>),
        );
        app.add_systems(
            Update,
            feed_app_search_results.run_if(resource_changed::<AppSearchResult>),
        );
        app.add_systems(
            Update,
            feed_file_search_results.run_if(resource_changed::<FileSearchResult>),
        );
        app.add_systems(
            Update,
            update_search_results_ui.run_if(resource_changed::<AppSearchResultUiResource>),
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

fn search_text_updated(
    mut action_events: EventWriter<MxSearchActionEvent>,
    mut app_search_feed: ResMut<AppSearchResultUiResource>,
    search_text: Res<SearchText>,
) {
    println!("Search text updated: {}", search_text.0);
    // On earch search first clear existing search results
    if search_text.0.is_empty() {
        println!("SEARCH TEXT IS EMPTY");
        app_search_feed.0.clear();
    }
    action_events.write(MxSearchActionEvent(MxSearchAction::SearchApplications(
        search_text.0.clone(),
    )));

    action_events.write(MxSearchActionEvent(MxSearchAction::SearchFiles(
        search_text.0.clone(),
    )));
}

fn feed_app_search_results(
    app_search_result: Res<AppSearchResult>,
    mut app_search_feed: ResMut<AppSearchResultUiResource>,
    asset_server: Res<AssetServer>,
) {
    let apps = app_search_result.0.clone();
    println!("Final Result of search: {:?}", apps);
    let mut final_results: Vec<SearchResult> = Vec::new();
    //TODO: revisit for svg icon issue, currently it's configured with default icon only
    for app in apps {
        let result = SearchResult {
            name: app.name,
            icon: lookup_icon(&app.icon, &app._type, &asset_server),
            on_click: None,
            _type: universal_search::SearchResultType::App,
        };
        if !app_search_feed.0.iter().any(|r| r.name == result.name) {
            final_results.push(result);
        }
    }
    app_search_feed.0.extend(final_results);
}

fn feed_file_search_results(
    file_search_result: Res<FileSearchResult>,
    mut app_search_feed: ResMut<AppSearchResultUiResource>,
    asset_server: Res<AssetServer>,
) {
    let files = file_search_result.0.clone();
    println!("Final Result of search: {:?}", files);
    let mut final_results: Vec<SearchResult> = Vec::new();
    //TODO: revisit for svg icon issue, currently it's configured with default icon only
    for file in files {
        let result = SearchResult {
            name: file.name,
            icon: lookup_icon(&file.icon, &SearchResultType::File, &asset_server),
            on_click: None,
            _type: universal_search::SearchResultType::File,
        };
        if !app_search_feed.0.iter().any(|r| r.name == result.name) {
            final_results.push(result);
        }
    }
    app_search_feed.0.extend(final_results);
}
fn update_search_results_ui(
    mut commands: Commands,
    app_search_results: Res<AppSearchResultUiResource>,
    query: Query<(Entity, &Children), With<SearchResultsComponent>>,
) {
    if !app_search_results.is_changed() {
        return;
    }
    println!("length of a search result: {}", app_search_results.0.len());
    for (parent_entity, children) in &query {
        // Despawn all direct children (which are the result entries)
        for child in children.iter() {
            commands.entity(child).despawn();
        }
        // Spawn the new children
        for result in &app_search_results.0 {
            commands.entity(parent_entity).with_children(|parent| {
                parent.spawn(search_result_item(result.clone()));
            });
        }
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
    wireless_enabled: Res<WirelessEnabled>,
    bluetooth_enabled: Res<BluetoothEnabledStatus>,
) {
    spawn_status_bar_ui(
        &mut commands,
        &font_assets,
        &wireless_enabled,
        &bluetooth_enabled,
    );

    // spawn_homescreen_window(&mut commands, &asset_server, &font_assets);

    //  spawn_settings_drawer(&mut commands, &theme_manager);

    spawn_navigation_bar_window(&mut commands);
}

fn spawn_status_bar_ui(
    commands: &mut Commands,
    font_assets: &Res<FontAssets>,
    wireless_enabled: &Res<WirelessEnabled>,
    bluetooth_enabled: &Res<BluetoothEnabledStatus>,
) {
    //Spawn status bar
    let wireless_default_icon = if wireless_enabled.0 {
        Icon::WirelessNone
    } else {
        Icon::WirelessOff
    }
    .into();
    let bluetooth_default_icon: Icon = if bluetooth_enabled.0 {
        Icon::BluetoothNone
    } else {
        Icon::BluetoothOff
    }
    .into();
    commands.spawn((Camera2d, StatusBarWindow));
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![status_bar(
            &font_assets,
            wireless_default_icon,
            bluetooth_default_icon
        )],
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

fn lookup_icon(
    icon_name: &str,
    search_result_type: &SearchResultType,
    asset_server: &AssetServer,
) -> Handle<Image> {
    let icon = lookup(&icon_name)
        .with_size(84)
        .with_theme("Papirus")
        .find()
        .unwrap_or_default()
        .into_os_string()
        .into_string()
        .unwrap();
    let default_icon = match search_result_type {
        SearchResultType::App => Path::new("icons/default_app_icon.png"),
        SearchResultType::File => Path::new("icons/default_file_icon.png"),
    };

    let path = Path::new(&icon);
    match path.extension() {
        Some(ext) if ext == "svg" => asset_server.load(AssetPath::from_path(default_icon)),
        Some(ext) if ext == "png" => asset_server.load(AssetPath::from_path(path)),
        _ => {
            println!("Unsupported icon format: {:?}", path.extension());
            asset_server.load(AssetPath::from_path(default_icon))
        }
    }
}
