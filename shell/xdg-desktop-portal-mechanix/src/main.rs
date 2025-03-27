use anyhow::Error;
use gui::xdg_portal_handler::{XDGPortal, XDGPortalParams};
use interfaces::access::Access;
use interfaces::file_chooser::FileChooser;
use interfaces::notification::Notification;
use interfaces::settings::Settings;
use interfaces::wallpaper::Wallpaper;
use mctk_core::reexports::cosmic_text;
use mctk_core::AssetParams;
use mctk_smithay::xdg_shell::xdg_window;
use mctk_smithay::{WindowInfo, WindowOptions};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::task::JoinHandle;
use zbus::blocking::connection::Builder;
use zbus::Connection;
use zbus::{blocking::connection, conn};

pub mod connections;
pub mod gui;
pub mod interfaces;

// const DBUS_NAME: &str = "org.mechanix.services.portal";
const DBUS_NAME: &str = "org.mechanix.services";
const DBUS_PATH: &str = "/org/freedesktop/portal/desktop";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let connection = zbus::ConnectionBuilder::session()?
        .name(DBUS_NAME)?
        .serve_at(DBUS_PATH, Wallpaper {})?
        .serve_at(DBUS_PATH, Notification {})?
        .serve_at(DBUS_PATH, Settings {})?
        .serve_at(DBUS_PATH, FileChooser {})?
        .serve_at(DBUS_PATH, Access {})?
        .build()
        .await?;

    // let mut fonts = cosmic_text::fontdb::Database::new();

    // let mut assets: HashMap<String, AssetParams> = HashMap::new();
    // let svgs: HashMap<String, String> = HashMap::new();

    // let window_info = WindowInfo {
    //     id: "xdg-desktop-portal-filechooser".to_string(),
    //     title: "xdg-desktop-portal".to_string(),
    //     namespace: "xdg-desktop-portal".to_string(),
    // };

    // // Set the window options
    // let window_opts = WindowOptions {
    //     height: 480 as u32,
    //     width: 480 as u32,
    //     scale_factor: 1.0,
    // };

    // assets.insert(
    //     "fold_icon".to_string(),
    //     AssetParams::new(settings.icons.fold_icon),
    // );

    // assets.insert(
    //     "file_icon".to_string(),
    //     AssetParams::new(settings.icons.file_icon),
    // );

    // assets.insert(
    //     "arrow_icon".to_string(),
    //     AssetParams::new(settings.icons.arrow_icon),
    // );

    // assets.insert(
    //     "back_icon".to_string(),
    //     AssetParams::new(settings.icons.back_icon),
    // );

    // assets.insert(
    //     "add_icon".to_string(),
    //     AssetParams::new(settings.icons.add_icon),
    // );

    // assets.insert(
    //     "dots_icon".to_string(),
    //     AssetParams::new(settings.icons.dots_icon),
    // );

    // assets.insert(
    //     "pdf_icon".to_string(),
    //     AssetParams::new(settings.icons.pdf_icon),
    // );

    // assets.insert(
    //     "img_icon".to_string(),
    //     AssetParams::new(settings.icons.img_icon),
    // );

    // assets.insert(
    //     "unfold_dir_icon".to_string(),
    //     AssetParams::new(settings.icons.unfold_dir_icon),
    // );

    // let (mut app, mut event_loop, ..) =
    //     xdg_window::XdgWindow::open_blocking::<XDGPortal, XDGPortalParams>(
    //         xdg_window::XdgWindowParams {
    //             window_info,
    //             window_opts,
    //             fonts,
    //             assets,
    //             svgs,
    //             ..Default::default()
    //         },
    //         XDGPortalParams {},
    //     );

    loop {
        std::future::pending::<()>().await; //comment this line if mctk app is running, lines below are uncommented

        // if app.is_exited {
        //     break;
        // }
        // event_loop
        //     .dispatch(Duration::from_millis(16), &mut app)
        //     .unwrap();
    }

    Ok(())
}
