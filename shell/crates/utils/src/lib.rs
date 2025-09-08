mod desktop_apps;
mod font_assets;

pub mod prelude {
    pub use crate::desktop_apps::{DesktopApp, DesktopApps, DesktopAppsPlugin};
    pub use crate::font_assets::{FontAssetsPlugin, fonts_loaded};
}
