use bevy::{color::palettes::css::RED, prelude::*};
use crate::components::NotificationButton;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &NotificationButton,
        ),
        (Changed<Interaction>, With<Button>, With<NotificationButton>),
    >,
) {
    for (interaction, mut color, mut border_color, notification) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Log the notification interaction
                info!(
                    "Notification pressed - ID: {}, Title: '{}', Content: '{}'",
                    notification.id,
                    notification.title,
                    notification.content
                );
                
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
        }
    }
}
