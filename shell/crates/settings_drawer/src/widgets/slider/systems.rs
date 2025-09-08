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
        (&CoreSlider, &Children, &StyledSlider, &mut BackgroundColor),
        (Or<(Added<StyledSlider>, Changed<Hovering>, Changed<CoreSlider>)>,),
    >,
    mut q_filled: Query<
        (&mut Node, &mut BackgroundColor, &mut BorderRadius),
        (Without<StyledSlider>, Without<Children>),
    >,

    mut image_query: Query<&mut ImageNode>,
) {
    // Get styles from theme manager
    let slider_styles = theme_manager.styles.slider.clone();

    for (slider_state, children, slider, mut bg_color) in q_slider.iter_mut() {
        let Some(filled_id) = children.first() else {
            warn!("Slider does not have a filled entity.");
            continue;
        };

        let Some(icon_id) = children.last() else {
            warn!("Slider does not have an icon entity.");
            continue;
        };

        let Ok((mut node, mut filled_bg_color, mut border_radius)) = q_filled.get_mut(*filled_id)
        else {
            warn!("Slider filled node not found.");
            continue;
        };

        let filled_width = Val::Percent(slider_state.thumb_position() * 100.0);
        if node.width != filled_width {
            node.width = filled_width;
        }
        if filled_width == Val::Percent(100.) {
            *border_radius = BorderRadius::all(Val::Px(12.));
        } else {
            *border_radius = BorderRadius {
                top_left: Val::Px(12.),
                top_right: Val::Auto,
                bottom_left: Val::Px(12.),
                bottom_right: Val::Auto,
            };
        }

        bg_color.0 = slider_styles.track_color;
        filled_bg_color.0 = slider_styles.filled_color;

        if let Ok(mut image_node) = image_query.get_mut(*icon_id) {
            let Some(icon) = slider.icon.clone() else {
                continue;
            };
            let Some(layout) = slider.layout.clone() else {
                continue;
            };

            *image_node = ImageNode::from_atlas_image(icon, TextureAtlas::from(layout));
        }
    }
}
