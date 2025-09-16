use bevy::{ prelude::*, window::{ CompositeAlphaMode, WindowResolution } };
use bevy_wayland::prelude::*;
use crate::components::cards::{ NotificationSurfaceEntity, NotificationSurface,ClearAllButton };
use crate::ui::ui;
use bevy_core_widgets::CoreScrollArea;

pub fn setup(mut commands: Commands) {
    let width = 540.0;
    let height = 576.0;

    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                transparent: true,
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::BOTTOM,
                layer: Layer::Top,
                exclusive_zone: 0,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            InputRegion(Rect::new(0.0, 0.0, width, height)),
        ))
        .id();
    let camera_ent = commands
        .spawn((
            Camera2d,
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(window_ent)
                ),
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
        ))
        .id();
    let notification_surface_entity = commands
        .spawn((
            UiTargetCamera(camera_ent),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart, // <-- Start at top
                padding: UiRect::all(Val::Px(16.0)),
                column_gap: Val::Px(16.0),
                overflow: Overflow::scroll_y(), // <-- Enable vertical scrolling
                ..Default::default()
            },
            CoreScrollArea,
            ScrollPosition {
                offset_x: 0.0,
                offset_y: 0.0,
            },
            BackgroundColor(Color::BLACK),
            NotificationSurface,
            children![(
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    // padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        top: Val::Px(7.0),
                        bottom: Val::Px(10.0),
                        left: Val::ZERO,
                        right: Val::ZERO,
                    },
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    ..default()
                },
                BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
                BorderRadius::all(Val::Px(8.0)),
                children![
                    (
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(30.0),
                            // margin: UiRect::left(Val::Px(10.0)),
                            ..default()
                        },
                        children![(
                            Text::new("Notifications"),
                            TextFont { font_size: 24.0, ..default() },
                            TextColor(Color::WHITE),
                        )],
                    ),
                    (
                        Button,
                        Node {
                            width: Val::Px(78.0),
                            height: Val::Px(30.0),
                            // top: Val::Px(8.0),
                            justify_content: JustifyContent::Center, // <-- Center horizontally
                            align_items: AlignItems::Center,
                            // margin: UiRect::right(Val::Px(2.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(8.0)),
                        ClearAllButton,
                        BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
                        children![(
                            Text::new("Clear all"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::WHITE),
                        )],
                    )
                ],
            )],
        ))
        .id();
    commands.insert_resource(NotificationSurfaceEntity(notification_surface_entity));

    // commands.spawn((UiTargetCamera(camera_ent), ui(&commands)));
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
