use bevy::prelude::*;

use app_drawer::prelude::*;
use background::prelude::*;
use homescreen::prelude::*;
use lockscreen::prelude::*;
use navigation_bar::prelude::*;
use notifications_drawer::prelude::*;
use running_apps::prelude::*;
use settings_drawer::prelude::*;
use settings_drawer::SettingsDrawerPlugin;
use status_bar::prelude::*;
use universal_search::prelude::*;

pub struct LauncherPlugin;
impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BackgroundPlugin,
            StatusBarPlugin,
            SettingsDrawerPlugin,
            // HomescreenPlugin,
            // RunningAppsPlugin,
            // UniversalSearchPlugin,
            // NotificationsDrawerPlugin,
        ));
    }
}

pub mod prelude {
    pub use crate::LauncherPlugin;
}
