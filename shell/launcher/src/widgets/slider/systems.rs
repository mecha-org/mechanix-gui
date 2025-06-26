use bevy::prelude::*;
use bevy_core_widgets::{CoreSlider, ValueChange, hover::Hovering};
use bevy_styled_widgets::prelude::ThemeManager;

use super::StyledSlider;

pub fn on_slide(
    mut trigger: Trigger<ValueChange<f32>>,
    mut q_slider: Query<&mut CoreSlider>,
    query: Query<&StyledSlider>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    let entity = trigger.target();
    let value = trigger.event().0;

    if let Ok(mut slider) = q_slider.get_mut(trigger.target()) {
        slider.set_value(trigger.event().0);
    }

    if let Ok(styled_slider) = query.get(entity) {
        if let Some(system_id) = styled_slider.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, value);
        }
    }
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
pub fn update_slider(
    theme_manager: Res<ThemeManager>,
    mut q_slider: Query<
        (&CoreSlider, &Children, &mut BackgroundColor),
        (
            With<StyledSlider>,
            Or<(Added<StyledSlider>, Changed<Hovering>, Changed<CoreSlider>)>,
        ),
    >,
    mut q_filled: Query<
        (&mut Node, &mut BackgroundColor),
        (Without<StyledSlider>, Without<Children>),
    >,

    mut q_text_color: Query<&mut TextColor, (Without<StyledSlider>, Without<Children>)>,
) {
    // Get styles from theme manager
    let slider_styles = theme_manager.styles.slider.clone();

    for (slider_state, children, mut bg_color) in q_slider.iter_mut() {
        let Some(filled_id) = children.first() else {
            warn!("Slider does not have a filled entity.");
            continue;
        };

        let Some(icon_id) = children.last() else {
            warn!("Slider does not have an icon entity.");
            continue;
        };

        let Ok((mut node, mut filled_bg_color)) = q_filled.get_mut(*filled_id) else {
            warn!("Slider filled node not found.");
            continue;
        };

        let filled_width = Val::Percent(slider_state.thumb_position() * 100.0);
        if node.width != filled_width {
            node.width = filled_width;
        }

        bg_color.0 = slider_styles.track_color;
        filled_bg_color.0 = slider_styles.filled_color;

        if let Ok(mut text_color) = q_text_color.get_mut(*icon_id) {
            text_color.0 = slider_styles.icon_color;
        }
    }
}
