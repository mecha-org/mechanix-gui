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
                Lockscreen::new(cx)
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
                _ => {}
            }
        }
    })
    .detach();
        println!("Lockscreen dispatcher-------------");
}
