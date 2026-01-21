use dispatcher::Dispatcher;
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use settings::prelude::{LayerShellSettings, LockscreenSettings, Settings};

mod ui;
use crate::ui::*;

pub mod prelude {
    pub use crate::listen_dispatcher;
    pub use crate::run_app;
}

pub fn run_app(cx: &mut App) {
    let LockscreenSettings { layer_shell } = Settings::global(cx).lockscreen.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = layer_shell;
    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: Some(exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let regions = Vec::new();
            window.set_input_regions(Some(regions));

            cx.new(|cx| {
                listen_dispatcher(cx);
                let lockscreen = Lockscreen::new(cx);
                // Fetch the default wallpaper asynchronously
                cx.spawn(async move |this: WeakEntity<Lockscreen> , cx| {
                    let setting_key = "org.mechanix.desktop.settings.lockscreen.wallpaper";
                    if let Ok(settings) = mxconf_dbus::get_setting(setting_key).await {
                        // mxconf_dbus::get_setting returns a HashMap<String, String>
                        // The value is usually stored under the key name or "value"
                        if let Some(wallpaper_path) = settings.get(setting_key) {
                            this.update(cx, |this, cx| {
                                this.wallpaper_path = std::path::PathBuf::from(wallpaper_path);
                                cx.notify();
                            }).ok();
                        }
                    }
                }).detach();
                lockscreen
            })
        },
    )
    .unwrap();
}

pub fn listen_dispatcher(cx: &mut Context<Lockscreen>) {
    println!("Lockscreen listening to dispatcher");

    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

        println!("Lockscreen dispatcher ready to receive messages");

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    cx.spawn(async move |this, cx| {
        println!("Lockscreen spawn...");
        while let Ok(message) = dispatcher_rx.recv().await {
            println!("while Lockscreen received message: {:#?}", message);
            match message {
                dispatcher::Message::ShowLockscreen(show) => {
                    let _ = this.update(cx, |this, cx| {
                        this.show = show;
                        if show {
                            this.reset(cx);
                        }
                        cx.notify();
                    });
                }
                dispatcher::Message::SetLockscreenWallpaper(wallpaper) => {
                    println!("wallpaper to set in lockscreen: {}", wallpaper);
                    let _ = this.update(cx, |this, cx| {
                        this.wallpaper_path = std::path::PathBuf::from(wallpaper);
                        cx.notify();
                    });
                }
                _ => {}
            }
        }
    })
    .detach();
        println!("Lockscreen dispatcher-------------");
}
