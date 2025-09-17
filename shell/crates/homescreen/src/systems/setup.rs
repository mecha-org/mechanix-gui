use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode, Viewport},
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    text::FontSmoothing,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};

use crate::{
    HomescreenCamera, HomescreenEvent, HomescreenSettings, HomescreenWidgetInfo, HomescreenWindow,
};

pub fn setup(mut commands: Commands) {
    let width = 540.;
    let height = 576.;

    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                transparent: true,
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::empty(),
                layer: Layer::Bottom,
                exclusive_zone: 0,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            HomescreenWindow,
            InputRegion(Rect::new(0., 0., width, height)),
        ))
        .id();

    commands.spawn((
        Camera2d,
        RenderLayers::layer(1),
        HomescreenCamera,
        MeshPickingCamera,
        Camera {
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(
                window_ent,
            )),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct ClockWidget(Handle<Image>);
#[derive(Component)]
pub struct ClockCamera(Handle<Image>);
pub fn update_clock_node_size(
    mut nodes: Query<(&mut Node, &ClockWidget)>,
    mut cameras: Query<(&mut Camera, &ClockCamera)>,
    images: Res<Assets<Image>>,
) {
    // for (mut node, clock) in &mut nodes {
    //     if let Some(image) = images.get(&clock.0) {
    //         node.height = Val::Px(image.height() as f32);
    //         node.width = Val::Px(image.width() as f32);
    //     }
    // }
    // for (mut camera, mut projection, clock) in &mut cameras {
    //     if let Some(image) = images.get(&clock.0) {
    //         camera.viewport = Some(Viewport {
    //             physical_position: UVec2::ZERO,
    //             physical_size: image.size(),
    //             depth: 0.0..1.0,
    //         });
    //         camera.target = clock.0.clone().into();
    //         *projection = Projection::Orthographic(OrthographicProjection {
    //             near: 10.0,
    //             far: -10.0,
    //             viewport_origin: Vec2::ZERO,
    //             scaling_mode: ScalingMode::FixedVertical {
    //                 viewport_height: image.height() as f32,
    //             },
    //             scale: 1.0,
    //             area: default(),
    //         });
    //     }
    // }
}

pub fn update_time(mut text: Single<&mut Text>) {
    let now_utc = Utc::now();
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    let now_ist = now_utc.with_timezone(&ist_offset);
    let formatted_time = now_ist.format("%H:%M").to_string();
    let formatted_day = now_ist.format("%a").to_string();

    text.0 = format!("{}", formatted_day);
}

pub fn spawn_widgets(
    mut commands: Commands,
    mut event_writer: EventWriter<HomescreenEvent>,
    mut images: ResMut<Assets<Image>>,
    mut asset_server: Res<AssetServer>,
    settings: Res<HomescreenSettings>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: 1000,
            height: 1000,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::bevy_default(),
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);
    let camera = commands
        .spawn((
            Camera2d,
            ClockCamera(image_handle.clone()),
            Camera {
                target: RenderTarget::Image(image_handle.clone().into()),
                order: -1,
                clear_color: ClearColorConfig::None,
                viewport: Some(Viewport {
                    physical_position: UVec2::ZERO,
                    physical_size: (1000, 1000).into(),
                    depth: 0.0..1.0,
                }),
                ..Default::default()
            },
            // Projection::Orthographic(OrthographicProjection {
            //     near: 10.0,
            //     far: -10.0,
            //     viewport_origin: Vec2::ZERO,
            //     scaling_mode: ScalingMode::FixedHorizontal {
            //         viewport_width: 250.0,
            //     },
            //     scale: 1.0,
            //     area: default(),
            // }),
        ))
        .id();
    let font = asset_server.load("font.ttf");
    let now_utc = Utc::now();
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    let now_ist = now_utc.with_timezone(&ist_offset);
    let formatted_time = now_ist.format("%H:%M").to_string();
    let formatted_day = now_ist.format("%a").to_string();
    commands.spawn((
        Node {
            width: Val::Px(1000.0),
            height: Val::Px(1000.0),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ClockWidget(image_handle.clone()),
        BackgroundColor(Color::srgb_u8(230, 126, 0)),
        UiTargetCamera(camera),
        children![
            (
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(-5.0), Val::Px(0.0)),
                    ..Default::default()
                },
                Text::new(formatted_time),
                TextColor(Color::BLACK),
                TextFont {
                    font: font.clone(),
                    font_size: 175.0,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Left),
            ),
            (
                Node {
                    margin: UiRect::new(Val::Px(50.0), Val::Px(0.0), Val::Px(-50.0), Val::Px(0.0)),
                    ..Default::default()
                },
                Text::new(formatted_day),
                TextColor(Color::BLACK),
                TextFont {
                    font: font.clone(),
                    font_size: 75.0,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Left),
            )
        ],
    ));
    event_writer.write(HomescreenEvent::CreateWidget {
        location: settings.grid_to_screen(&(0, 0).into()),
        info: HomescreenWidgetInfo {
            size: (1, 1).into(),
            name: "Small Icon".into(),
            color: Color::srgb(0.25, 0.25, 0.25),
            image_handle: None,
        },
    });
    event_writer.write(HomescreenEvent::CreateWidget {
        location: settings.grid_to_screen(&(1, 0).into()),
        info: HomescreenWidgetInfo {
            size: (1, 1).into(),
            name: "Small Icon".into(),
            color: Color::srgb(0.25, 0.25, 0.25),
            image_handle: None,
        },
    });
    event_writer.write(HomescreenEvent::CreateWidget {
        location: settings.grid_to_screen(&(2, 0).into()),
        info: HomescreenWidgetInfo {
            size: (2, 1).into(),
            name: "Small Icon".into(),
            color: Color::WHITE,
            image_handle: Some(image_handle.clone()),
        },
    });
    event_writer.write(HomescreenEvent::CreateWidget {
        location: settings.grid_to_screen(&(0, 1).into()),
        info: HomescreenWidgetInfo {
            size: (4, 3).into(),
            name: "Small Icon".into(),
            color: Color::srgb(0.25, 0.25, 0.25),
            image_handle: None,
        },
    });
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
