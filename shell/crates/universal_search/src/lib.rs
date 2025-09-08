use bevy::{
    asset::{AssetMetaCheck, AssetPath},
    ecs::system::SystemId,
    prelude::*,
};

mod components;
mod icons;
mod mock;
mod resources;
mod states;
mod systems;
mod types;
mod ui;

use systems::*;
use types::*;
use utils::prelude::{FontAssetsPlugin, fonts_loaded};

use crate::{
    icons::{UniversalSearchIconsPlugin, icons_loaded},
    resources::IsOpen,
    states::{Action, listen_action},
    ui::{
        BrowserApps, FrequentlyUsedApps, SearchInputPlugin, SearchItems, SearchResults, SearchText,
    },
};
use headless_widgets::prelude::*;

#[derive(Event)]
pub struct UniversalSearchOpen;

#[derive(Event)]
pub struct UniversalSearchClose;

pub struct UniversalSearchPlugin;
impl Plugin for UniversalSearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FrequentlyUsedApps(vec![]));
        app.insert_resource(SearchItems(vec![]));
        app.insert_resource(SearchResults(vec![]));
        app.insert_resource(SearchText("".to_string()));
        app.insert_resource(BrowserApps(vec![]));

        app.add_plugins(FontAssetsPlugin);
        app.add_plugins(UniversalSearchIconsPlugin);
        app.add_plugins((animation::DefaultTweenPlugins,));
        app.add_plugins((mock::MockPlugin,));
        app.add_plugins((headless_widgets::CoreWidgetsPlugin));
        app.add_plugins(SearchInputPlugin);

        app.add_event::<UniversalSearchOpen>();
        app.add_event::<UniversalSearchClose>();
        app.add_systems(Startup, camera_setup);
        app.add_systems(
            Update,
            setup
                .run_if(resource_exists::<UniversalSearchWindowCamera>)
                .run_if(fonts_loaded)
                .run_if(icons_loaded),
        );

        app.add_observer(listen_open_event);
        app.add_observer(listen_close_event);
        app.add_systems(Update, listen_close_completed);
        app.add_systems(Update, (button_system, exit_on_esc));

        // app.insert_resource(IsOpen(false));
        // app.add_systems(Update, (button_system, effect_system, exit_on_esc));
        // app.add_event::<Action>();

        app.add_observer(on_bar_drag_start);
        app.add_observer(on_bar_drag);
        app.add_observer(on_bar_drag_end);

        // app.add_systems(Update, listen_action);
    }
}

pub mod prelude {
    pub use crate::UniversalSearchPlugin;
    pub use crate::{UniversalSearchClose, UniversalSearchOpen};
}
