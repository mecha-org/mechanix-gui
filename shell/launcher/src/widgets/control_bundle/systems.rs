use bevy::prelude::*;
use bevy_core_widgets::{ButtonPressed, InteractionDisabled, hover::Hovering};
use bevy_styled_widgets::prelude::ThemeManager;

use super::{
    ButtonSize,
    components::{ControlVariant, StyledControl, StyledControlText},
};

// Update the button's background color.
#[allow(clippy::type_complexity)]
pub fn update_button(
    theme_manager: Res<ThemeManager>,
    children: Query<&mut Children>,
    mut text_query: Query<(&mut Text, &mut TextColor, &mut TextFont), With<StyledControlText>>,
    mut query: Query<(
        Entity,
        &mut Node,
        &StyledControl,
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
        ButtonPressed(is_pressed),
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
                if let Ok((mut text, mut text_color, mut text_font)) = text_query.get_mut(child) {
                    let button_styles = theme_manager.styles.buttons.clone();
                    let button_size_styles = theme_manager.styles.button_sizes.clone();
                    let button_style = match button.variant {
                        ControlVariant::Primary => button_styles.primary,
                        ControlVariant::Secondary => button_styles.secondary,
                        ControlVariant::Destructive => button_styles.destructive,
                        ControlVariant::Outline => button_styles.outline,
                        ControlVariant::Ghost => button_styles.ghost,
                    };
                    let color = button_style.text_color;
                    text_color.0 = color;

                    //update font size
                    let button_size_style = match button.size.unwrap_or_default() {
                        ButtonSize::XSmall => button_size_styles.xsmall,
                        ButtonSize::Small => button_size_styles.small,
                        ButtonSize::Medium => button_size_styles.medium,
                        ButtonSize::Large => button_size_styles.large,
                        ButtonSize::XLarge => button_size_styles.xlarge,
                    };
                    text_font.font_size = button_size_style.font_size;

                    //update text
                    if let Some(text_str) = button.text.clone() {
                        text.0 = text_str.clone();
                    }

                    //update icon
                    if let Some(icon) = button.icon.clone() {
                        if let Some(theme_icon) = theme_icons.get(&icon) {
                            text.0 = theme_icon.clone();
                        } else {
                            text.0 = icon;
                        };
                    }

                    //update font
                    if let Some(font) = &button.font {
                        text_font.font = font.clone();
                    }
                }
            }
        };

        // Update the background color based on the button's state
        let button_style = match button.variant {
            ControlVariant::Primary => button_styles.primary,
            ControlVariant::Secondary => button_styles.secondary,
            ControlVariant::Destructive => button_styles.destructive,
            ControlVariant::Outline => button_styles.outline,
            ControlVariant::Ghost => button_styles.ghost,
        };

        match (is_disabled, is_pressed, is_hovering) {
            (true, _, _) => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
            (_, true, true) => {
                bg_color.0 = button_style.pressed_background;
                border_color.0 = button_style.border_color;
            }
            (_, false, true) => {
                bg_color.0 = button_style.hovered_background;
                border_color.0 = button_style.border_color;
            }
            _ => {
                bg_color.0 = button_style.normal_background;
                border_color.0 = button_style.border_color;
            }
        };

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
        border_radius.top_left = Val::Px(button_size_style.border_radius);
        border_radius.top_right = Val::Px(button_size_style.border_radius);
        border_radius.bottom_left = Val::Px(button_size_style.border_radius);
        border_radius.bottom_right = Val::Px(button_size_style.border_radius);

        if button.border_radius.is_some() {
            border_radius.top_left = Val::Px(button.border_radius.unwrap_or_default());
            border_radius.top_right = Val::Px(button.border_radius.unwrap_or_default());
            border_radius.bottom_left = Val::Px(button.border_radius.unwrap_or_default());
            border_radius.bottom_right = Val::Px(button.border_radius.unwrap_or_default());
        }
    }
}
