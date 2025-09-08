use bevy::prelude::*;
use bevy_core_widgets::{ButtonPressed, InteractionDisabled, hover::Hovering};
use bevy_styled_widgets::prelude::ThemeManager;

use crate::widgets::button::AnimationConfig;

use super::{
    ButtonSize,
    components::{ButtonVariant, StyledButton, StyledButtonText},
};

// Update the button's background color.
#[allow(clippy::type_complexity)]
pub fn update_button(
    theme_manager: Res<ThemeManager>,
    children: Query<&mut Children>,
    mut image_query: Query<&mut ImageNode>,
    mut query: Query<(
        Entity,
        &mut Node,
        &StyledButton,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut BorderRadius,
        &Hovering,
        &ButtonPressed,
        Has<InteractionDisabled>,
    )>,
) {
    for (
        button_entity_id,
        mut button_node,
        button,
        mut bg_color,
        mut border_color,
        mut border_radius,
        Hovering(is_hovering),
        ButtonPressed { is_pressed, .. },
        is_disabled,
    ) in query.iter_mut()
    {
        // Get styles from theme manager
        let button_styles = theme_manager.styles.buttons.clone();
        let button_size_styles = theme_manager.styles.button_sizes.clone();
        let theme_icons = theme_manager.styles.icons.clone();

        // Update text
        //Get button text
        if let Ok(children) = children.get(button_entity_id) {
            for child in children.iter() {
                if let Ok(mut image_node) = image_query.get_mut(child) {
                    let Some(icon) = button.icon.clone() else {
                        continue;
                    };
                    let Some(layout) = button.layout.clone() else {
                        continue;
                    };

                    *image_node = ImageNode::from_atlas_image(icon, TextureAtlas::from(layout));
                }
            }
        };

        // Update the background color based on the button's state
        let button_style = match button.variant {
            ButtonVariant::Primary => button_styles.primary,
            ButtonVariant::Secondary => button_styles.secondary,
            ButtonVariant::Destructive => button_styles.destructive,
            ButtonVariant::Outline => button_styles.outline,
            ButtonVariant::Ghost => button_styles.ghost,
        };

        // let is_active = button.active.unwrap_or(false);
        // let active_background = button
        //     .active_background_color
        //     .unwrap_or(button_style.normal_background);

        // match (is_disabled, is_pressed, is_hovering) {
        //     (true, _, _) => {
        //         bg_color.0 = button_style.normal_background;
        //         border_color.0 = button_style.border_color;
        //     }
        //     (_, true, true) => {
        //         bg_color.0 = button_style.pressed_background;
        //         border_color.0 = button_style.border_color;
        //     }
        //     (_, false, true) => {
        //         bg_color.0 = button_style.hovered_background;
        //         border_color.0 = button_style.border_color;
        //     }
        //     _ => {
        //         bg_color.0 = button_style.normal_background;
        //         border_color.0 = button_style.border_color;
        //     }
        // };

        let is_active = button.active.unwrap_or(false);

        match (is_disabled, is_active, is_pressed, is_hovering) {
            (true, _, _, _) => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
            (false, true, false, _) => {
                bg_color.0 = button
                    .active_background_color
                    .unwrap_or(button_style.active_background);
                border_color.0 = button_style.border_color;
            }
            (false, true, true, _) => {
                bg_color.0 = button_style.pressed_background;
                border_color.0 = button_style.border_color;
            }
            (false, false, true, true) => {
                bg_color.0 = button_style.pressed_background;
                border_color.0 = button_style.border_color;
            }
            (false, false, false, true) => {
                bg_color.0 = button_style.hovered_background;
                border_color.0 = button_style.border_color;
            }
            _ => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
        }

        //Update size styles
        let button_size_style = match button.size.unwrap_or_default() {
            ButtonSize::XSmall => button_size_styles.xsmall,
            ButtonSize::Small => button_size_styles.small,
            ButtonSize::Medium => button_size_styles.medium,
            ButtonSize::Large => button_size_styles.large,
            ButtonSize::XLarge => button_size_styles.xlarge,
        };

        button_node.padding = UiRect::axes(
            Val::Px(button_size_style.padding_horizontal),
            Val::Px(button_size_style.padding_vertical),
        );
        button_node.border = UiRect::all(Val::Px(button_size_style.border_width));
        // border_radius.top_left = Val::Px(button_size_style.border_radius);
        // border_radius.top_right = Val::Px(button_size_style.border_radius);
        // border_radius.bottom_left = Val::Px(button_size_style.border_radius);
        // border_radius.bottom_right = Val::Px(button_size_style.border_radius);

        // if button.border_radius.is_some() {
        //     border_radius.top_left = Val::Px(button.border_radius.unwrap_or_default());
        //     border_radius.top_right = Val::Px(button.border_radius.unwrap_or_default());
        //     border_radius.bottom_left = Val::Px(button.border_radius.unwrap_or_default());
        //     border_radius.bottom_right = Val::Px(button.border_radius.unwrap_or_default());
        // }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut ImageNode)>,
) {
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
            }
        }
    }
}
