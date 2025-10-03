use crate::init_services_nav;
use crate::modules::running_apps::app_manager::AppManagerMessage;
use crate::navigation_bar_ui;
use crate::AppMessage;
use crate::AppParams;
use crate::InitServicesParamsNav;
use crate::UiParams;
use mctk_core::context;
use mctk_core::reexports::smithay_client_toolkit::shell::wlr_layer;
use mctk_core::{
    msg,
    reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Event},
};
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window::LayerWindow;
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::WindowInfo;
use mctk_smithay::WindowMessage;
use mctk_smithay::WindowOptions;
use tokio::sync::mpsc;

pub fn launch_navigation_bar(ui_params: UiParams) -> anyhow::Result<()> {
    let UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
        ..
    } = ui_params;

    let window_opts = WindowOptions {
        height: 300 as u32,
        width: 520 as u32,
        scale_factor: 1.0,
    };

    //subscribe to events channel
    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();

    let app_id = String::from("mechanix.shell.navigation-bar");
    let namespace = app_id.clone();
    let mut layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::None,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let input_region = Some((180, 260, 160, 40));

    let (mut app, mut event_loop, window_tx) =
        LayerWindow::open_blocking::<navigation_bar_ui::NavigationBar, AppParams>(
            LayerWindowParams {
                window_info,
                window_opts,
                fonts,
                assets,
                layer_shell_opts: layer_shell_opts.clone(),
                input_region,
                svgs,
                layer_tx: None,
                layer_rx: None,
            },
            AppParams {
                app_channel: Some(app_channel_tx.clone()),
                ..Default::default()
            },
        );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let window_tx_3 = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        window_tx_3
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));

    // let (wireless_msg_tx, wireless_msg_rx) = mpsc::channel(128);
    let (app_manager_msg_tx, app_manager_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::MinimizeAll => {
                    //Cllose all windows
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let _ = app_manager_msg_tx2
                            .send(AppManagerMessage::MinimizeAll)
                            .await;
                    });
                }
                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services_nav(InitServicesParamsNav {
        app_channel: app_channel_tx,
        app_manager_msg_rx,
    });

    loop {
        if app.is_exited {
            break;
        }

        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End
    Ok(())
}
