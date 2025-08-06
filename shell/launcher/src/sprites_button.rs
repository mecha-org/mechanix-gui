use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};
use bevy_core_widgets::{ButtonPressed, CoreButton, InteractionDisabled, hover::Hovering};

use crate::utils::FontAssets;

#[derive(Component)]
pub struct Button;

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct Wireless(bool);

pub fn sprites_button_demo(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_assets: &FontAssets,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(css::DARK_GRAY.into()),
        children![button(commands, asset_server, font_assets)],
    )
}

fn button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_assets: &FontAssets,
) -> impl Bundle {
    let on_click = commands.register_system(
        |mut image_nodes: Query<(&mut AnimationConfig, &mut ImageNode, &mut Wireless)>,
         font_assets: Res<FontAssets>| {
            for (mut animation, mut image_node, mut wireless) in &mut image_nodes {
                if wireless.0 {
                    *image_node = ImageNode::from_atlas_image(
                        font_assets.wireless_disable.clone(),
                        TextureAtlas::from(font_assets.wireless_disable_layout.clone()),
                    );
                } else {
                    *image_node = ImageNode::from_atlas_image(
                        font_assets.wireless_enable.clone(),
                        TextureAtlas::from(font_assets.wireless_enable_layout.clone()),
                    );
                }
                animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
                wireless.0 = !wireless.0
            }
        },
    );

    (
        Node {
            width: Val::Px(200.),
            height: Val::Px(80.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        CoreButton {
            on_click: Some(on_click),
        },
        Button,
        BackgroundColor(css::BLACK.into()),
        BorderRadius::all(Val::Px(10.)),
        children![(
            ImageNode::from_atlas_image(
                font_assets.wireless_enable.clone(),
                TextureAtlas::from(font_assets.wireless_enable_layout.clone()),
            ),
            Node {
                width: Val::Px(80.),
                height: Val::Px(80.),
                ..default()
            },
            Wireless(true),
            AnimationConfig::new(0, 29, 60),
        )],
    )
}

pub struct SpritesButtonPlugin;

impl Plugin for SpritesButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button_styles);
        app.add_systems(Update, execute_animations);
    }
}

fn update_button_styles(mut query: Query<(&mut BackgroundColor, &ButtonPressed), With<Button>>) {
    for (mut bg, ButtonPressed(pressed)) in query.iter_mut() {
        if *pressed {
            bg.0 = css::GRAY.into();
        } else {
            bg.0 = css::BLACK.into();
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut ImageNode)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then stop
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
                println!("atlas.index {}", atlas.index);
            }
        }
    }
}

// fn increment_atlas_index(
//     time: Res<Time>,
//     mut image_nodes: Query<(&mut AnimationTimer, &mut ImageNode)>,
// ) {
//     for (mut timer, mut image_node) in &mut image_nodes {
//         timer.tick(time.delta());

//         if timer.just_finished() {
//             if let Some(atlas) = &mut image_node.texture_atlas {
//                 atlas.index = (atlas.index + 1) % 30;
//             }
//         }
//     }
// }
