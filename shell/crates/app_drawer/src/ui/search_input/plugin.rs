use bevy::prelude::*;

use crate::ui::search_input::systems::{listen_keyboard_input_events, on_click, on_close_click};
use crate::ui::search_input::{SearchActive, SearchText};

pub struct SearchInputPlugin;

impl Plugin for SearchInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SearchText("".to_string()));
        app.insert_resource(SearchActive(false));
        app.add_observer(on_click);
        app.add_observer(on_close_click);
        app.add_systems(Update, listen_keyboard_input_events);
    }
}
