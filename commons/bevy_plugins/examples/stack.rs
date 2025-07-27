use bevy::{ prelude::*, winit::WinitSettings };

mod stack_plugin;
use stack_plugin::CardStackPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(CardStackPlugin)
        .run();
}
