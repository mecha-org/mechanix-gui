use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
mod core_button;
mod core_scrollbar;
mod core_slider;
mod events;

mod interaction_states;
pub use core_button::{CoreButton, CoreButtonPlugin};
pub use core_scrollbar::{
    CoreScrollArea, CoreScrollbar, CoreScrollbarPlugin, CoreScrollbarThumb, Orientation,
};
pub use core_slider::{CoreSlider, CoreSliderPlugin, SliderDragState};
pub use events::{ButtonClicked, ValueChange};
pub use interaction_states::{ButtonPressed, Checked, InteractionDisabled};
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<InputDispatchPlugin>() {
            app.add_plugins(InputDispatchPlugin);
        }

        app.add_plugins((CoreButtonPlugin, CoreScrollbarPlugin, CoreSliderPlugin));
    }
}

pub mod prelude {
    pub use crate::core_button::{CoreButton, CoreButtonPlugin};
}
