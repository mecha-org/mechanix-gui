use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::oklch(0.2891, 0., 0.);
pub const PRESSED_BUTTON: Color = Color::oklch(0.3942, 0., 0.);

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
