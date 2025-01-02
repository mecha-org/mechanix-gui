use gui::{FileManager, FileManagerParams};
use mctk_core::reexports::cosmic_text;
use mctk_core::AssetParams;
use mctk_smithay::xdg_shell::xdg_window;
use mctk_smithay::{WindowInfo, WindowOptions};
use settings::MainSettings;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

mod constants;
mod errors;
mod folder_options;
mod gui;
mod modals;
mod settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match crate::settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);
            MainSettings::default()
        }
    };
    // Initialize the font database
    let mut fonts = cosmic_text::fontdb::Database::new();
    for path in settings.fonts.paths.clone() {
        if let Ok(content) = fs::read(Path::new(&path)) {
            fonts.load_font_data(content);
        }
    }

    // Initialize the asset manager
    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let namespace = settings.app.id.clone().unwrap_or_default();

    // Set the window info
    let window_info = WindowInfo {
        id: settings.app.id.clone().unwrap_or_default(),
        title: settings.title.clone(),
        namespace,
    };

    // Set the window options
    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    // Add your assets here
    assets.insert(
        "fold_icon".to_string(),
        AssetParams::new(settings.icons.fold_icon),
    );

    assets.insert(
        "file_icon".to_string(),
        AssetParams::new(settings.icons.file_icon),
    );

    assets.insert(
        "arrow_icon".to_string(),
        AssetParams::new(settings.icons.arrow_icon),
    );

    assets.insert(
        "back_icon".to_string(),
        AssetParams::new(settings.icons.back_icon),
    );

    assets.insert(
        "add_icon".to_string(),
        AssetParams::new(settings.icons.add_icon),
    );

    assets.insert(
        "dots_icon".to_string(),
        AssetParams::new(settings.icons.dots_icon),
    );

    assets.insert(
        "pdf_icon".to_string(),
        AssetParams::new(settings.icons.pdf_icon),
    );

    assets.insert(
        "img_icon".to_string(),
        AssetParams::new(settings.icons.img_icon),
    );

    assets.insert(
        "unfold_dir_icon".to_string(),
        AssetParams::new(settings.icons.unfold_dir_icon),
    );

    let (mut app, mut event_loop, ..) =
        xdg_window::XdgWindow::open_blocking::<FileManager, FileManagerParams>(
            xdg_window::XdgWindowParams {
                window_info,
                window_opts,
                fonts,
                assets,
                svgs,
                ..Default::default()
            },
            FileManagerParams {},
        );

    loop {
        if app.is_exited {
            break;
        }

        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    Ok(())
}
