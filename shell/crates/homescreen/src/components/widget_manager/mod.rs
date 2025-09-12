use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Hash)]
pub struct Widget {
    id: u32,
    width: u32,
    height: u32,
}

pub struct WidgetManager {
    widget_to_entity: HashMap<Widget, Entity>,
    entity_to_widget: HashMap<Entity, Widget>,
    rows: u32,
    cols: u32,
}
impl WidgetManager {}

pub struct WidgetManagerPlugin;
impl Plugin for WidgetManagerPlugin {
    fn build(&self, app: &mut App) {}
}
