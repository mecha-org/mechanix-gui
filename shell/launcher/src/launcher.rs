use crate::utils::Icon;
use crate::{
    // StyledWidgetsPlugin,
    components::{status_bar, AssetsLoadingState, ClockPlugin},
    styled_card::StyledCardPlugin,
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

pub fn run_launcher() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        io: TaskPoolThreadAssignmentPolicy {
                            min_threads: 9,
                            max_threads: 10,
                            percent: 0.5,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        ..Default::default()
                    }
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Status Bar".to_string(),
                        resolution: (540.0, 44.0).into(),
                        ..default()
                    }),
                    ..default()
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
            // SmithayPlugin {
            //     primary_window_type: SmithayWindowType::LayerShell {
            //         settings: LayerShellSettings::default(),
            //     },
            // },
            // StyledWidgetsPlugin,
            StyledTextPlugin,
        ))
        .insert_resource(ThemeManager::default())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup)
        .add_plugins(ClockPlugin)
        .add_plugins(StyledCardPlugin)
        .add_plugins(NetworkManagerPlugin)
        .add_plugins(BluetoothPlugin)
        .add_plugins(UPowerPlugin)
        .add_observer(bar_on_click)
        .add_observer(bar_on_drag_start)
        .add_observer(bar_on_drag)
        .add_observer(bar_on_drag_end)
        .add_systems(Update, animate_bar_drag_end)
        .add_systems(Update, exit_on_esc)
        .run();
}

#[derive(Debug, PartialEq, Eq)]
pub enum BarPos {
    Left,
    Center,
    Right,
}

#[derive(Debug, Component)]
pub struct Bar(BarPos);

#[derive(Debug, Component)]
pub struct BarDragEnd(Val, f32);

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

    //Spawn apps list window
    let apps_list_entity = commands
        .spawn((
            Window {
                title: "Apps list".to_string(),
                resolution: (540., 531.).into(),
                ..default()
            },
            SmithayWindowType::LayerShell {
                settings: LayerShellSettings {
                    size: (540, 531),
                    layer: bevy_smithay::prelude::subsurface::Layer::Bottom,
                    anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
                    exclusive_zone: 0,
                    ..Default::default()
                },
            },
        ))
        .id();

    let apps_list_window_camera = commands
        .spawn((
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(apps_list_entity),
                ),
                clear_color: ClearColorConfig::Custom(Color::default()),
                ..default()
            },
            Camera2d,
        ))
        .id();

    commands.spawn((
        UiTargetCamera(apps_list_window_camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        children![apps_grid(),],
    ));

    //Spawn bottom bar
    let bottom_bar_entity = commands
        .spawn((
            Window {
                title: "Bottom bar".to_string(),
                resolution: (540., 44.).into(),
                ..default()
            },
            SmithayWindowType::LayerShell {
                settings: LayerShellSettings {
                    size: (540, 44),
                    layer: bevy_smithay::prelude::subsurface::Layer::Top,
                    anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::BOTTOM,
                    exclusive_zone: 44,
                    ..Default::default()
                },
            },
        ))
        .id();

    let bottom_bar_window_camera = commands
        .spawn((
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(bottom_bar_entity),
                ),
                clear_color: ClearColorConfig::Custom(Color::default()),
                ..default()
            },
            Camera2d,
        ))
        .id();

    commands.spawn((
        UiTargetCamera(bottom_bar_window_camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        children![bottom_bar(),],
    ));

    // UiTargetCamera(window_camera);

    // bottom_bar(),
}

fn apps_grid() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.)),
            ..Default::default()
        },
        BorderColor(Color::linear_rgba(0., 0., 0., 0.2)),
        children![(
            Text::new("Apps / Widgets surface"),
            TextFont {
                font_size: 24.,
                ..Default::default()
            },
            TextColor(Color::BLACK)
        )],
    )
}

fn bottom_bar() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![
            bottom_bar_item(BarPos::Left),
            bottom_bar_item(BarPos::Center),
            bottom_bar_item(BarPos::Right)
        ],
    )
}

const BAR_BOTTOM_DISTANCE: f32 = 12.;

fn bottom_bar_item(pos: BarPos) -> impl Bundle {
    let mut width = Val::Percent(25.);
    let height = Val::Percent(100.);
    if pos == BarPos::Center {
        width = Val::Percent(50.);
    }

    (
        Node {
            width,
            height,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(0.5)),
            ..Default::default()
        },
        BorderColor(Color::linear_rgba(0., 0., 0., 0.2)),
        children![(
            Node {
                width: Val::Percent(70.),
                height: Val::Px(8.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(BAR_BOTTOM_DISTANCE),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(8.)),
            BackgroundColor(Color::linear_rgb(0.74, 0.74, 0.74)),
            Bar(pos)
        )],
    )
}

fn bar_on_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_state: Query<(&mut Node, &Bar)>,
) {
    trigger.propagate(false);
    if let Ok((mut node, Bar(pos))) = q_state.get_mut(trigger.target) {
        println!("bar drag started {:?}", pos);
    }
}

fn bar_on_drag(mut trigger: Trigger<Pointer<Drag>>, mut q_state: Query<&mut Node, With<Bar>>) {
    trigger.propagate(false);
    if let Ok(mut node) = q_state.get_mut(trigger.target) {
        let distance = trigger.distance.y.abs();
        node.bottom = Val::Px(BAR_BOTTOM_DISTANCE + distance);
    }
}

fn bar_on_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    mut q_state: Query<&Node, With<Bar>>,
    time: Res<Time>,
) {
    trigger.propagate(false);
    if let Ok(node) = q_state.get_mut(trigger.target) {
        commands
            .entity(trigger.target)
            .insert(BarDragEnd(node.bottom, time.elapsed_secs()));
    }
}

fn bar_on_click(mut trigger: Trigger<Pointer<Click>>, mut q_state: Query<(&Bar)>) {
    trigger.propagate(false);
    if let Ok((Bar(pos))) = q_state.get_mut(trigger.target) {
        println!("bar clicked {:?}", pos);
    }
}

fn animate_bar_drag_end(
    mut commands: Commands,
    mut q_state: Query<(Entity, &mut Node, &BarDragEnd)>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    for (entity, mut node, BarDragEnd(drag_ended_at, start_time)) in q_state.iter_mut() {
        let start = match drag_ended_at {
            Val::Px(val) => *val,
            _ => 0.,
        };
        let end = BAR_BOTTOM_DISTANCE;

        let animation_duration = 0.5;
        let elapsed = now - start_time;

        let progress = (elapsed / animation_duration).min(1.0);

        if progress >= 1.0 {
            commands.entity(entity).remove::<BarDragEnd>();
            continue;
        }

        let eased_progress = progress * progress * progress;

        let new_bottom = start + (end - start) * eased_progress;

        node.bottom = Val::Px(new_bottom);
    }
}
